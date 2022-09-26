pub fn bw_server(nblisteners: &f32, bitrate: &f32) {
    let compute =
        |nblisteners: &f32, bitrate: &f32| -> f32 { 125.0 * nblisteners * bitrate / 128.0 };

    println!(
        "Number of listeners: {0}\
        \nBitrate (kb/s): {1}\
        \nServer bandwidth (Mib/s): {2}",
        nblisteners,
        bitrate,
        compute(nblisteners, bitrate)
    );
}

pub fn server_usage_bw(nblisteners: &f32, bitrate: &f32, nbdays: &f32, nbhours: &f32) {
    let compute = |nbdays: &f32, nbhours: &f32, bitrate: &f32, nblisteners: &f32| -> f32 {
        28125.0 * nbdays * nbhours * bitrate * nblisteners / 65536.0
    };
    println!(
        "Number of listeners: {0}\
        \nBitrate (kb/s): {1}\
        \nNumber of days: {2}\
        \nNumber of hours by days: {3}\
        \nBandwidth used (GiB): {4}",
        nblisteners,
        bitrate,
        nbdays,
        nbhours,
        compute(nblisteners, bitrate, nbdays, nbhours)
    );
}

#[test]
fn tests_bw_server() {
    assert_eq!(
        print!("Number of listeners: 250 \nBitrate (kb/s): 64 \nServer bandwidth (Mib/s): 15625"),
        bw_server(&250.0, &64.00)
    );
    assert_eq!(
        print!(
            "Number of listeners: 250 \nBitrate (kb/s): 64.8 \nServer bandwidth (Mib/s): 15820.313"
        ),
        bw_server(&250.0, &64.88)
    );
}

#[test]
fn tests_server_usage_bw() {
    assert_eq!(print!("Number of listeners: 250 \nBitrate (kb/s): 64 \nNumber of days: 1 \nNumber of hours by days: 24 \n Bandwidth used (GiB): 164794.92"), server_usage_bw(&250.0, &64.0, &1.0, &24.0));
    assert_eq!(print!("Number of listeners: 250 \nBitrate (kb/s): 64.8 \nNumber of days: 1 \nNumber of hours by days: 24 \nBandwidth used (GiB): 166854.88"), server_usage_bw(&250.0, &64.8, &1.0, &24.0));
}
