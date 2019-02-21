enum ServiceState {
    CREATE_JUST,
    RUNNING,
    SHUTDOWN_ALREADY,
    START_FAILED
}

pub(crate) struct Client{
    name_server: String,
    name_server_domain: String,
    instance_name:String,
    group_name:String,
    service_state:ServiceState
}

impl Client{
    
}