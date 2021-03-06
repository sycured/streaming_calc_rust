pub fn bw_server(nblisteners: f32, bitrate: f32) {
    let compute = |nblisteners: f32, bitrate: f32| -> String {
        (125.0 * nblisteners * bitrate / 128.0).to_string()
    };

    format!(
        "Number of listeners: {nblisteners} \nBitrate (kb/s): {bitrate} \nServer bandwidth (Mib/s): {x}",
        nblisteners=nblisteners, bitrate=bitrate, x=compute(nblisteners, bitrate)
    );
}

pub fn server_usage_bw(nblisteners: f32, bitrate: f32, nbdays: f32, nbhours: f32) {
    let compute = |nbdays: f32, nbhours: f32, bitrate: f32, nblisteners: f32| -> String {
        (28125.0 * nbdays * nbhours * bitrate * nblisteners / 65536.0).to_string()
    };

    format!(
        "Number of listeners: {nblisteners} \nBitrate (kb/s): {bitrate} \nNumber of days: {nbdays} \nNumber of hours by days: {nbhours} \nBandwidth used (GiB): {x}",
        nblisteners=nblisteners, bitrate=bitrate, nbdays=nbdays, nbhours=nbhours, x=compute(nblisteners, bitrate, nbdays, nbhours)
    );
}

#[test]
fn tests_bw_server() {
    assert_eq!(
        print!("Number of listeners: 250 \nBitrate (kb/s): 64 \nServer bandwidth (Mib/s): 15625"),
        bw_server(250.0, 64.0)
    );
    assert_eq!(
        print!(
            "Number of listeners: 250 \nBitrate (kb/s): 64.8 \nServer bandwidth (Mib/s): 15820.313"
        ),
        bw_server(250.0, 64.8)
    );
}

#[test]
fn tests_server_usage_bw() {
    assert_eq!(print!("Number of listeners: 250 \nBitrate (kb/s): 64 \nNumber of days: 1 \nNumber of hours by days: 24 \n Bandwidth used (GiB): 164794.92"), server_usage_bw(250.0, 64.0, 1.0, 24.0));
    assert_eq!(print!("Number of listeners: 250 \nBitrate (kb/s): 64.8 \nNumber of days: 1 \nNumber of hours by days: 24 \nBandwidth used (GiB): 166854.88"), server_usage_bw(250.0,64.8,1.0,24.0));
}
