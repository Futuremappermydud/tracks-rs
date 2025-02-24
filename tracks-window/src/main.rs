use tracks_rs::ffi::*;

mod graphing;

fn main() {
    let array = unsafe {
        tracks_create_json_array(
            [
                tracks_create_json_array(
                    [
                        tracks_create_json_number(0.0),
                        tracks_create_json_number(0.0),
                    ]
                    .as_ptr(),
                    2,
                ),
                tracks_create_json_array(
                    [
                        tracks_create_json_number(1.0),
                        tracks_create_json_number(1.0),
                    ]
                    .as_ptr(),
                    2,
                ),
            ]
            .as_ptr(),
            2,
        )
    };
    let context = unsafe { tracks_make_base_provider_context() };
    let float_point_definition = unsafe { tracks_make_float_point_definition(&array, context) };
    println!("float_point_definition: {:?}", unsafe {
        tracks_interpolate_float(float_point_definition, 0.5, context).value
    });

    graphing::graph("2d");
    // graphing::graph("3d");
    // graphing::graph("color");
    //graphing::graph("quat");
}
