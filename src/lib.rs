pub fn bw_server(nblisteners: f32, bitrate: f32) {
    if nblisteners == 0.0 || bitrate == 0.0 {
        eprintln!("The number of listeners and bitrate must be non-zero.");
    } else {
        let compute = 125.0f64 * (nblisteners as f64) * (bitrate as f64) / 128.0;
        if compute <= (f32::MAX as f64) {
            println!(
                "Number of listeners: {0}\
            \nBitrate (kb/s): {1}\
            \nServer bandwidth (Mib/s): {2}",
                nblisteners, bitrate, compute as f32
            );
        } else {
            eprintln!("arithmetic overflow on floating-point multiplication: f32::MAX")
        }
    }
}

pub fn server_usage_bw(nblisteners: f32, bitrate: f32, nbdays: f32, nbhours: f32) {
    if nblisteners == 0.0 || bitrate == 0.0 || nbdays == 0.0 || nbhours == 0.0 {
        eprintln!("The number of listeners, bitrate, number of days, and number of hours must be non-zero.");
    } else {
        let compute = 28125.0f64
            * (nbdays as f64)
            * (nbhours as f64)
            * (bitrate as f64)
            * (nblisteners as f64)
            / 65536.0;
        if compute <= (f32::MAX as f64) {
            println!(
                "Number of listeners: {0}\
            \nBitrate (kb/s): {1}\
            \nNumber of days: {2}\
            \nNumber of hours by days: {3}\
            \nBandwidth used (GiB): {4}",
                nblisteners, bitrate, nbdays, nbhours, compute as f32
            );
        } else {
            eprintln!("arithmetic overflow on floating-point multiplication: f32::MAX")
        }
    }
}

#[test]
fn tests_bw_server() {
    assert_eq!(
        print!("Number of listeners: 250 \nBitrate (kb/s): 64 \nServer bandwidth (Mib/s): 15625"),
        bw_server(250.0, 64.00)
    );
    assert_eq!(
        print!(
            "Number of listeners: 250 \nBitrate (kb/s): 64.8 \nServer bandwidth (Mib/s): 15820.313"
        ),
        bw_server(250.0, 64.88)
    );
}

#[test]
fn tests_server_usage_bw() {
    assert_eq!(print!("Number of listeners: 250 \nBitrate (kb/s): 64 \nNumber of days: 1 \nNumber of hours by days: 24 \n Bandwidth used (GiB): 164794.92"), server_usage_bw(250.0, 64.0, 1.0, 24.0));
    assert_eq!(print!("Number of listeners: 250 \nBitrate (kb/s): 64.8 \nNumber of days: 1 \nNumber of hours by days: 24 \nBandwidth used (GiB): 166854.88"), server_usage_bw(250.0, 64.8, 1.0, 24.0));
}

#[cfg(kani)]
#[kani::proof]
fn kani_bw_server() {
    let input1: f32 = kani::any();
    kani::assume(input1.is_sign_positive());
    let input2: f32 = kani::any();
    kani::assume(input2.is_sign_positive());
    bw_server(input1, input2)
}
#[cfg(kani)]
#[kani::proof]
fn kani_server_usage_bw() {
    let input1: f32 = kani::any();
    kani::assume(input1.is_sign_positive());
    let input2: f32 = kani::any();
    kani::assume(input2.is_sign_positive());
    let input3: f32 = kani::any();
    kani::assume(input3.is_sign_positive());
    let input4: f32 = kani::any();
    kani::assume(input4.is_sign_positive());
    server_usage_bw(input1, input2, input3, input4)
}
