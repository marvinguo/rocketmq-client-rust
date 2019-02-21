enum ServiceState {
    CREATE_JUST,
    RUNNING,
    SHUTDOWN_ALREADY,
    START_FAILED
}

pub(crate) struct Client{
    //ip+port
    name_server: String,
    //先只支持ip:port, domain:port的，以后一律不区分
    //name_server_domain: String,
    instance_name:String,
    group_name:String,
    service_state:ServiceState
}

impl Client{

}