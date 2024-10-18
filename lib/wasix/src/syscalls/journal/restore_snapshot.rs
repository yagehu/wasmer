use super::*;

/// Safety: This function manipulates the memory of the process and thus must
/// be executed by the WASM process thread itself.
///
#[cfg(feature = "journal")]
#[allow(clippy::result_large_err)]
#[tracing::instrument(skip_all)]
pub unsafe fn restore_snapshot(
    mut ctx: FunctionEnvMut<'_, WasiEnv>,
    journal: Arc<DynJournal>,
    bootstrapping: bool,
) -> Result<Option<RewindState>, WasiRuntimeError> {
    use std::{collections::BTreeMap, ops::Range};

    use crate::{journal::Journal, os::task::process::MemorySnapshotRegion};

    // Create the journal replay runner
    let mut runner = JournalSyscallPlayer::new(ctx, bootstrapping);

    // We read all the logs from the journal into the state machine
    let mut ethereal_events = Vec::new();
    while let Some(next) = journal.read().map_err(anyhow_err_to_runtime_err)? {
        tracing::trace!(event=?next, "restoring event");
        runner.play_event(next.into_inner(), Some(&mut ethereal_events));
    }

    // Check for events that are orphaned
    for evt in ethereal_events {
        tracing::trace!("Orphaned ethereal events - {:?}", evt);
    }

    // Now output the stdout and stderr
    tracing::trace!("replaying stdout");
    for (offset, data, is_64bit) in runner.stdout {
        if is_64bit {
            JournalEffector::apply_fd_write::<Memory64>(&runner.ctx, 1, offset, data)
        } else {
            JournalEffector::apply_fd_write::<Memory32>(&runner.ctx, 1, offset, data)
        }
        .map_err(anyhow_err_to_runtime_err)?;
    }

    tracing::trace!("replaying stdout");
    for (offset, data, is_64bit) in runner.stderr {
        if is_64bit {
            JournalEffector::apply_fd_write::<Memory64>(&runner.ctx, 2, offset, data)
        } else {
            JournalEffector::apply_fd_write::<Memory32>(&runner.ctx, 2, offset, data)
        }
        .map_err(anyhow_err_to_runtime_err)?;
    }

    // Apply the memory changes (if this is in bootstrapping mode we differed them)
    for (region, data) in runner.differ_memory {
        tracing::trace!(
            "Replay journal - UpdateMemory - region:{:?}, data.len={}",
            region,
            data.len()
        );
        JournalEffector::apply_compressed_memory(&mut runner.ctx, region, &data)
            .map_err(anyhow_err_to_runtime_err)?;
    }

    // Reset the freed FD's so that none of them will be reused after
    // the journal resumes
    runner.ctx.data().state().fs.clear_freed_fd_list();

    // Once we get to this point we are no longer replaying the journal
    // and need to clear this flag, the reason is that restoring the
    // background threads may immediately process requests while this
    // flag is still set which would be bad
    tracing::trace!("replaying journal=false");
    runner.ctx.data_mut().replaying_journal = false;

    // Spawn all the threads
    let thread_count = runner.spawn_threads.len();
    tracing::trace!(thread_count, "restoring threads");
    for (index, (thread_id, thread_state)) in runner.spawn_threads.into_iter().enumerate() {
        tracing::trace!("restoring thread {}/{}", index + 1, thread_count);

        if thread_state.is_64bit {
            JournalEffector::apply_thread_state::<Memory64>(
                &mut runner.ctx,
                thread_id,
                thread_state.memory_stack,
                thread_state.rewind_stack,
                thread_state.store_data,
                thread_state.start,
                thread_state.layout,
            )
            .map_err(anyhow_err_to_runtime_err)?;
        } else {
            JournalEffector::apply_thread_state::<Memory32>(
                &mut runner.ctx,
                thread_id,
                thread_state.memory_stack,
                thread_state.rewind_stack,
                thread_state.store_data,
                thread_state.start,
                thread_state.layout,
            )
            .map_err(anyhow_err_to_runtime_err)?;
        }
    }
    tracing::debug!(thread_count, "snapshot restore complete");

    Ok(runner.rewind)
}
