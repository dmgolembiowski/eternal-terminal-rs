use cpp_inherit::*;
#[inherit_from(Console)]
#[derive(Debug)]
struct RustConsole{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/terminal/Console.hpp")]
impl RustConsole {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(UserTerminalRouter)]
#[derive(Debug)]
struct RustUserTerminalRouter{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/terminal/UserTerminalRouter.hpp")]
impl RustUserTerminalRouter {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(UserTerminal)]
#[derive(Debug)]
struct RustUserTerminal{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/terminal/UserTerminal.hpp")]
impl RustUserTerminal {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(UserTerminalHandler)]
#[derive(Debug)]
struct RustUserTerminalHandler{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/terminal/UserTerminalHandler.hpp")]
impl RustUserTerminalHandler {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(SshSetupHandler)]
#[derive(Debug)]
struct RustSshSetupHandler{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/terminal/SshSetupHandler.hpp")]
impl RustSshSetupHandler {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(Client;)]
#[derive(Debug)]
struct RustClient;{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/terminal/TelemetryService.hpp")]
impl RustClient; {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(TelemetryService)]
#[derive(Debug)]
struct RustTelemetryService{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/terminal/TelemetryService.hpp")]
impl RustTelemetryService {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(TerminalClient)]
#[derive(Debug)]
struct RustTerminalClient{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/terminal/TerminalClient.hpp")]
impl RustTerminalClient {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(ProcessHelper)]
#[derive(Debug)]
struct RustProcessHelper{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/terminal/ProcessHelper.hpp")]
impl RustProcessHelper {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(PsuedoUserTerminal)]
#[derive(Debug)]
struct RustPsuedoUserTerminal{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/terminal/PsuedoUserTerminal.hpp")]
impl RustPsuedoUserTerminal {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(TerminalServer)]
#[derive(Debug)]
struct RustTerminalServer{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/terminal/TerminalServer.hpp")]
impl RustTerminalServer {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(PsuedoTerminalConsole)]
#[derive(Debug)]
struct RustPsuedoTerminalConsole{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/terminal/PsuedoTerminalConsole.hpp")]
impl RustPsuedoTerminalConsole {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(UserJumphostHandler)]
#[derive(Debug)]
struct RustUserJumphostHandler{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/terminal/UserJumphostHandler.hpp")]
impl RustUserJumphostHandler {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(PortForwardHandler)]
#[derive(Debug)]
struct RustPortForwardHandler{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/terminal/forwarding/PortForwardHandler.hpp")]
impl RustPortForwardHandler {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(ForwardDestinationHandler)]
#[derive(Debug)]
struct RustForwardDestinationHandler{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/terminal/forwarding/ForwardDestinationHandler.hpp")]
impl RustForwardDestinationHandler {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(ForwardSourceHandler)]
#[derive(Debug)]
struct RustForwardSourceHandler{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/terminal/forwarding/ForwardSourceHandler.hpp")]
impl RustForwardSourceHandler {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(ClientConnection)]
#[derive(Debug)]
struct RustClientConnection{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/base/ClientConnection.hpp")]
impl RustClientConnection {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(ServerConnection)]
#[derive(Debug)]
struct RustServerConnection{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/base/ServerConnection.hpp")]
impl RustServerConnection {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(RawSocketUtils)]
#[derive(Debug)]
struct RustRawSocketUtils{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/base/RawSocketUtils.hpp")]
impl RustRawSocketUtils {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(BackedWriterWriteState)]
#[derive(Debug)]
struct RustBackedWriterWriteState{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/base/BackedWriter.hpp")]
impl RustBackedWriterWriteState {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(BackedWriter)]
#[derive(Debug)]
struct RustBackedWriter{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/base/BackedWriter.hpp")]
impl RustBackedWriter {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(PipeSocketHandler)]
#[derive(Debug)]
struct RustPipeSocketHandler{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/base/PipeSocketHandler.hpp")]
impl RustPipeSocketHandler {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(ServerClientConnection)]
#[derive(Debug)]
struct RustServerClientConnection{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/base/ServerClientConnection.hpp")]
impl RustServerClientConnection {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(LogHandler)]
#[derive(Debug)]
struct RustLogHandler{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/base/LogHandler.hpp")]
impl RustLogHandler {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(Packet)]
#[derive(Debug)]
struct RustPacket{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/base/Packet.hpp")]
impl RustPacket {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(DaemonCreator)]
#[derive(Debug)]
struct RustDaemonCreator{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/base/DaemonCreator.hpp")]
impl RustDaemonCreator {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(WinsockContext)]
#[derive(Debug)]
struct RustWinsockContext{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/base/WinsockContext.hpp")]
impl RustWinsockContext {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(BackedReader)]
#[derive(Debug)]
struct RustBackedReader{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/base/BackedReader.hpp")]
impl RustBackedReader {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(Connection)]
#[derive(Debug)]
struct RustConnection{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/base/Connection.hpp")]
impl RustConnection {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(TcpSocketHandler)]
#[derive(Debug)]
struct RustTcpSocketHandler{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/base/TcpSocketHandler.hpp")]
impl RustTcpSocketHandler {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(SocketHandler)]
#[derive(Debug)]
struct RustSocketHandler{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/base/SocketHandler.hpp")]
impl RustSocketHandler {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(CryptoHandler)]
#[derive(Debug)]
struct RustCryptoHandler{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/base/CryptoHandler.hpp")]
impl RustCryptoHandler {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(UnixSocketHandler)]
#[derive(Debug)]
struct RustUnixSocketHandler{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/base/UnixSocketHandler.hpp")]
impl RustUnixSocketHandler {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(IpcPairClient)]
#[derive(Debug)]
struct RustIpcPairClient{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/htm/IpcPairClient.hpp")]
impl RustIpcPairClient {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(HtmServer)]
#[derive(Debug)]
struct RustHtmServer{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/htm/HtmServer.hpp")]
impl RustHtmServer {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(IpcPairEndpoint)]
#[derive(Debug)]
struct RustIpcPairEndpoint{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/htm/IpcPairEndpoint.hpp")]
impl RustIpcPairEndpoint {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(IpcPairServer)]
#[derive(Debug)]
struct RustIpcPairServer{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/htm/IpcPairServer.hpp")]
impl RustIpcPairServer {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(MultiplexerState)]
#[derive(Debug)]
struct RustMultiplexerState{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/htm/MultiplexerState.hpp")]
impl RustMultiplexerState {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(TerminalHandler)]
#[derive(Debug)]
struct RustTerminalHandler{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/htm/TerminalHandler.hpp")]
impl RustTerminalHandler {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
#[inherit_from(HtmClient)]
#[derive(Debug)]
struct RustHtmClient{}

#[inherit_from_impl(BaseType, "./src/EternalTerminal/src/htm/HtmClient.hpp")]
impl RustHtmClient {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
