use super::*;
use crate::syscalls::*;

/// ### `sock_join_multicast_v4()`
/// Joins a particular multicast IPv4 group
///
/// ## Parameters
///
/// * `fd` - Socket descriptor
/// * `multiaddr` - Multicast group to joined
/// * `interface` - Interface that will join
#[instrument(level = "trace", skip_all, fields(%sock), ret)]
pub fn sock_join_multicast_v4<M: MemorySize>(
    mut ctx: FunctionEnvMut<'_, WasiEnv>,
    sock: WasiFd,
    multiaddr: WasmPtr<__wasi_addr_ip4_t, M>,
    iface: WasmPtr<__wasi_addr_ip4_t, M>,
) -> Result<Errno, WasiError> {
    let env = ctx.data();
    let memory = unsafe { env.memory_view(&ctx) };
    let multiaddr = wasi_try_ok!(crate::net::read_ip_v4(&memory, multiaddr));
    let iface = wasi_try_ok!(crate::net::read_ip_v4(&memory, iface));

    wasi_try_ok!(sock_join_multicast_v4_internal(
        &mut ctx, sock, multiaddr, iface
    )?);

    #[cfg(feature = "journal")]
    if ctx.data().enable_journal {
        JournalEffector::save_sock_join_ipv4_multicast(&mut ctx, sock, multiaddr, iface).map_err(
            |err| {
                tracing::error!("failed to save sock_join_ipv4_multicast event - {}", err);
                WasiError::Exit(ExitCode::Errno(Errno::Fault))
            },
        )?;
    }

    Ok(Errno::Success)
}

pub(crate) fn sock_join_multicast_v4_internal(
    ctx: &mut FunctionEnvMut<'_, WasiEnv>,
    sock: WasiFd,
    multiaddr: Ipv4Addr,
    iface: Ipv4Addr,
) -> Result<Result<(), Errno>, WasiError> {
    let env = ctx.data();
    wasi_try_ok_ok!(__sock_actor_mut(ctx, sock, Rights::empty(), |socket, _| {
        socket.join_multicast_v4(multiaddr, iface)
    }));
    Ok(Ok(()))
}
