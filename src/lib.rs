fn bw_server(nblisteners: f32, bitrate: f32) -> f32 {
    nblisteners * bitrate * 1000.0 / 1024.0
}

fn server_usage_bw(nblisteners: f32, bitrate: f32, nbdays: f32, nbhours: f32) -> f32 {
    nbdays * nbhours * 3600.0 * bitrate * 1000.0 / 8.0 * nblisteners / 1024.0 / 1024.0
}

pub fn print_bw_server(nblisteners: f32, bitrate: f32) {
    print!(
        "Number of listeners: {0} \nBitrate (kb/s): {1} \nServer bandwidth (Mib/s): {2}",
        nblisteners,
        bitrate,
        bw_server(nblisteners, bitrate).to_string()
    );
}

pub fn print_server_usage_bw(nblisteners: f32, bitrate: f32, nbdays: f32, nbhours: f32) {
    print!(
        "Number of listeners: {0} \nBitrate (kb/s): {1} \nNumber of days: {2} \nNumber of hours by days: {3} \nBandwidth used (GiB): {4}",
           nblisteners,
           bitrate,
           nbdays,
           nbhours,
           server_usage_bw(nblisteners, bitrate, nbdays, nbhours).to_string()
    );
}

#[test]
fn tests_bw_server() {
    assert_eq!(15625.0, bw_server(250.0, 64.0));
    assert_eq!(15820.313, bw_server(250.0, 64.8));
    assert_eq!(
        print!("Number of listeners: 250 \nBitrate (kb/s): 64 \nServer bandwidth (Mib/s): 15625"),
        print_bw_server(250.0, 64.0)
    );
    assert_eq!(
        print!(
            "Number of listeners: 250 \nBitrate (kb/s): 64.8 \nServer bandwidth (Mib/s): 15820.313"
        ),
        print_bw_server(250.0, 64.8)
    );
}

#[test]
fn tests_server_usage_bw() {
    assert_eq!(164794.92, server_usage_bw(250.0, 64.0, 1.0, 24.0));
    assert_eq!(166854.88, server_usage_bw(250.0, 64.8, 1.0, 24.0));
    assert_eq!(print!("Number of listeners: 250 \nBitrate (kb/s): 64 \nNumber of days: 1 \nNumber of hours by days: 24 \n Bandwidth used (GiB): 164794.92"), print_server_usage_bw(250.0, 64.0, 1.0, 24.0));
    assert_eq!(print!("Number of listeners: 250 \nBitrate (kb/s): 64.8 \nNumber of days: 1 \nNumber of hours by days: 24 \nBandwidth used (GiB): 166854.88"), print_server_usage_bw(250.0,64.8,1.0,24.0));
}
