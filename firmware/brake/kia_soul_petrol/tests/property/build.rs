extern crate gcc;
extern crate bindgen;

use std::env;
use std::path::Path;

fn main() {
    gcc::Config::new()
        .flag("-w")
        .define("KIA_SOUL", Some("ON"))
        .include("include")
        .include("../../include")
        .include("../../../../common/testing/mocks")
        .include("../../../../common/include")
        .include("../../../../common/libs/can")
        .include("../../../../common/libs/time")
        .include("../../../../common/libs/pid")
        .include("../../../../../api/include")
        .file("../../../../common/testing/mocks/Arduino_mock.cpp")
        .file("../../../../common/testing/mocks/mcp_can_mock.cpp")
        .file("../../src/communications.cpp")
        .file("../../src/brake_control.cpp")
        .file("../../src/globals.cpp")
        .file("../../src/master_cylinder.cpp")
        .file("../../src/helper.cpp")
        .file("../../../../common/libs/can/oscc_can.cpp")
        .cpp(true)
        .compile("libbrake_test.a");

    let out_dir = env::var("OUT_DIR").unwrap();

    let _ = bindgen::builder()
        .header("include/wrapper.hpp")
        .generate_comments(false)
        .clang_arg("-DKIA_SOUL=ON")
        .clang_arg("-I../../include")
        .clang_arg("-I../../../../common/testing/mocks")
        .clang_arg("-I../../../../common/include")
        .clang_arg("-I../../../../common/libs/can")
        .clang_arg("-I../../../../common/libs/pid")
        .clang_arg("-I../../../../../api/include")
        .whitelisted_function("publish_brake_report")
        .whitelisted_function("check_for_incoming_message")
        .whitelisted_function("check_for_operator_override")
        .whitelisted_var("OSCC_MAGIC_BYTE_0")
        .whitelisted_var("OSCC_MAGIC_BYTE_1")
        .whitelisted_var("OSCC_BRAKE_ENABLE_CAN_ID")
        .whitelisted_var("OSCC_BRAKE_DISABLE_CAN_ID")
        .whitelisted_var("OSCC_BRAKE_REPORT_CAN_ID")
        .whitelisted_var("OSCC_BRAKE_REPORT_CAN_DLC")
        .whitelisted_var("OSCC_BRAKE_COMMAND_CAN_ID")
        .whitelisted_var("OSCC_BRAKE_COMMAND_CAN_DLC")
        .whitelisted_var("BRAKE_OVERRIDE_PEDAL_THRESHOLD_IN_DECIBARS")
        .whitelisted_var("MINIMUM_BRAKE_COMMAND")
        .whitelisted_var("MAXIMUM_BRAKE_COMMAND")
        .whitelisted_var("CAN_STANDARD")
        .whitelisted_var("CAN_MSGAVAIL")
        .whitelisted_var("g_control_can")
        .whitelisted_var("g_brake_control_state")
        .whitelisted_type("oscc_brake_enable_s")
        .whitelisted_type("oscc_brake_disable_s")
        .whitelisted_type("oscc_brake_report_s")
        .whitelisted_type("oscc_brake_command_s")
        .whitelisted_type("can_frame_s")
        .generate()
        .unwrap()
        .write_to_file(Path::new(&out_dir).join("brake_test.rs"))
        .expect("Unable to generate bindings");
}
