use std::{fs, net, sync};

trait SyncSend {}
impl<T: Sync + Send> SyncSend for T {}

fn assert_sync_send<T: SyncSend>() {}

fn main() {
    assert_sync_send::<sync::Mutex<()>>();
    assert_sync_send::<sync::Condvar>();
    assert_sync_send::<sync::RwLock<()>>();
    assert_sync_send::<sync::Barrier>();
    assert_sync_send::<sync::Arc<()>>();
    assert_sync_send::<sync::Weak<()>>();
    assert_sync_send::<sync::Once>();

    assert_sync_send::<fs::File>();
    assert_sync_send::<fs::Metadata>();
    assert_sync_send::<fs::ReadDir>();
    assert_sync_send::<fs::DirEntry>();
    assert_sync_send::<fs::OpenOptions>();
    assert_sync_send::<fs::Permissions>();

    assert_sync_send::<net::TcpStream>();
    assert_sync_send::<net::TcpListener>();
    assert_sync_send::<net::UdpSocket>();
    assert_sync_send::<net::SocketAddr>();
    assert_sync_send::<net::SocketAddrV4>();
    assert_sync_send::<net::SocketAddrV6>();
    assert_sync_send::<net::Ipv4Addr>();
    assert_sync_send::<net::Ipv6Addr>();
}