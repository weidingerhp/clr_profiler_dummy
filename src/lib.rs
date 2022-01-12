use clr_profiler::{ClrProfiler, CorProfilerCallback, CorProfilerCallback2, CorProfilerCallback3, CorProfilerCallback4, CorProfilerCallback5, CorProfilerCallback6, CorProfilerCallback7, CorProfilerCallback8, CorProfilerCallback9, CorProfilerInfo, ProfilerInfo, ffi::{COR_PRF_MONITOR, HRESULT}, register};
use log::{info, warn};
use simple_logger::SimpleLogger;
use uuid::Uuid;

use clr_profiler::ffi::GUID;

#[derive(Clone)]
struct Profiler {
    clsid: Uuid,
    profiler_info: Option<ProfilerInfo>,
}
impl Profiler {
    fn profiler_info(&self) -> &ProfilerInfo {
        self.profiler_info.as_ref().unwrap()
    }
}
impl ClrProfiler for Profiler {
    fn new() -> Profiler {
        SimpleLogger::new().init().unwrap();
        Profiler {
            clsid: Uuid::parse_str("DF63A541-5A33-4611-8829-F4E495985EE3").unwrap(),
            profiler_info: None,
        }
    }
    fn clsid(&self) -> &Uuid {
        &self.clsid
    }
}
impl CorProfilerCallback for Profiler {
    fn initialize(&mut self, profiler_info: ProfilerInfo) -> Result<(), HRESULT> {
        info!("initialize called");
        // Initialize ICorProfilerInfo reference
        self.profiler_info = Some(profiler_info);

        // Set the event mask
        match self.profiler_info()
            .set_event_mask(COR_PRF_MONITOR::COR_PRF_MONITOR_REMOTING) {
                Ok(_) => { info!("Profiler Mask set"); Ok(()) },
                Err(h_res) => { warn!("Profiler mask set failed"); Err(h_res) },
            }?;

        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), HRESULT> {
        info!("Shutdown called");
        Ok(())
    }

    // Remoting Invocation Callbacks
    fn remoting_client_invocation_started(&mut self) -> Result<(), HRESULT> {
        info!("remoting_client_invocation_started");
        Ok(())
    }

    fn remoting_client_sending_message(
        &mut self,
        cookie: GUID,
        is_async: bool,
    ) -> Result<(), HRESULT> {
        info!("remoting_client_sending_message - cookie: {}, is_async: {}", cookie, is_async);
        Ok(())
    }

    fn remoting_client_receiving_reply(
        &mut self,
        cookie: GUID,
        is_async: bool,
    ) -> Result<(), HRESULT> {
        info!("remoting_client_receiving_reply - cookie: {}, is_async: {}", cookie, is_async);
        Ok(())
    }

    fn remoting_client_invocation_finished(&mut self) -> Result<(), HRESULT> {
        info!("remoting_client_invocation_finished");
        Ok(())
    }

    fn remoting_server_receiving_message(
        &mut self,
        cookie: GUID,
        is_async: bool,
    ) -> Result<(), HRESULT> {
        info!("remoting_server_receiving_message - cookie: {}, is_async: {}", cookie, is_async);
        Ok(())
    }

    fn remoting_server_invocation_started(&mut self) -> Result<(), HRESULT> {
        info!("remoting_server_invocation_started");
        Ok(())
    }

    fn remoting_server_invocation_returned(&mut self) -> Result<(), HRESULT> {
        info!("remoting_server_invocation_returned");
        Ok(())
    }

    fn remoting_server_sending_reply(
        &mut self,
        cookie: GUID,
        is_async: bool,
    ) -> Result<(), HRESULT> {
        info!("remoting_server_sending_reply - cookie: {}, is_async: {}", cookie, is_async);
        Ok(())
    }    
}

impl CorProfilerCallback2 for Profiler {}
impl CorProfilerCallback3 for Profiler {}
impl CorProfilerCallback4 for Profiler {}
impl CorProfilerCallback5 for Profiler {}
impl CorProfilerCallback6 for Profiler {}
impl CorProfilerCallback7 for Profiler {}
impl CorProfilerCallback8 for Profiler {}
impl CorProfilerCallback9 for Profiler {}

register!(Profiler);
