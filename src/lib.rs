#[allow(dead_code)]
mod lib_impp;

use jni::objects::{JObject, JString};
use jni::sys::{jarray, jobjectArray, jstring};
use jni::JNIEnv;
use std::convert::TryFrom;
use std::ffi::{CStr, CString};

pub static FILES_PATH: &str = "/data/user/0/com.example.android/files/";

// DEPRECATED FUNCTION
#[no_mangle]
pub unsafe extern "C" fn Java_com_example_android_MainActivity_hello(
    env: JNIEnv,
    _: JObject,
    j_recipient: JString,
) -> jstring {
    let recipient = CString::from(CStr::from_ptr(
        env.get_string(j_recipient).unwrap().as_ptr(),
    ));
    let output_string = lib_impp::input_to_output(recipient.to_str().unwrap().to_string());

    let output = env.new_string(output_string).unwrap();
    output.into_inner()
}

// Return Part of Java String
#[no_mangle]
pub unsafe extern "C" fn Java_com_example_android_MainActivity_getTitle(
    env: JNIEnv,
    _: JObject,
    j_recipient: JString,
) -> jstring {
    // convert JString to CString and append it to our JNIEnv
    env.new_string(lib_impp::return_title(
        CString::from(CStr::from_ptr(
            env.get_string(j_recipient).unwrap().as_ptr(),
        ))
        .to_str()
        .unwrap()
        .to_string(),
    ))
    .unwrap()
    .into_inner()
}

// Return a bool
#[no_mangle]
pub unsafe extern "C" fn Java_com_example_android_MainActivity_importfromGoogleSheet(
    env: JNIEnv,
    _: JObject,
    j_recipient: JString,
) -> bool {
    lib_impp::import_googlesheet(
        CString::from(CStr::from_ptr(
            env.get_string(j_recipient).unwrap().as_ptr(),
        ))
        .to_str()
        .unwrap()
        .to_string(),
        &FILES_PATH,
    )
}

// Return a i32 as number for a random question
#[no_mangle]
pub unsafe extern "C" fn Java_com_example_android_MainActivity_getRandomQuestion(
    env: JNIEnv,
    _: JObject,
    j_recipient: JString,
) -> i32 {
    lib_impp::generate_random_question(
        CString::from(CStr::from_ptr(
            env.get_string(j_recipient).unwrap().as_ptr(),
        ))
        .to_str()
        .unwrap()
        .to_string(),
        &FILES_PATH,
    )
}

// Return Array with a Question Element
#[no_mangle]
pub unsafe extern "C" fn Java_com_example_android_MainActivity_getQuestionDetails(
    env: JNIEnv,
    _: JObject,
    question_num: i32,
) -> jarray {
    let question_details_array: [String; 4] =
        lib_impp::get_question_details(question_num, &FILES_PATH);
    // Initialize our array with 4 empty Strings
    let array: jobjectArray = env
        .new_object_array(
            4,
            env.find_class("java/lang/String").unwrap(),
            *env.new_string("").unwrap(),
        )
        .unwrap();
    let mut i = 0;
    while i < 4 {
        // Edit every Item of the Array to give it the values we want
        env.set_object_array_element(
            array,
            i,
            *env.new_string(
                question_details_array
                    [usize::try_from(i).expect("Variable i could not be converted to usize.")]
                .to_owned(),
            )
            .unwrap()
            .to_owned(),
        )
        .expect("Could not perform set_object_array_element on array element.");
        i += 1;
    }
    array
}

// Return Array with Multiple-Choice Distractors
#[no_mangle]
pub unsafe extern "C" fn Java_com_example_android_MainActivity_getMCDistractors(
    env: JNIEnv,
    _: JObject,
    question_num: i32,
    jeopardy_mode: bool,
) -> jarray {
    let question_details_array: [String; 4] =
        lib_impp::get_mc_distractors(question_num, jeopardy_mode, &FILES_PATH);
    // Initialize our array with 4 empty Strings
    let array: jobjectArray = env
        .new_object_array(
            4,
            env.find_class("java/lang/String").unwrap(),
            *env.new_string("").unwrap(),
        )
        .unwrap();
    let mut i = 0;
    while i < 4 {
        // Edit every Item of the Array to give it the values we want
        env.set_object_array_element(
            array,
            i,
            *env.new_string(
                question_details_array
                    [usize::try_from(i).expect("Variable i could not be converted to usize.")]
                .to_owned(),
            )
            .unwrap()
            .to_owned(),
        )
        .expect("Could not perform set_object_array_element on array element.");
        i += 1;
    }
    array
}

/*
// Return Test Array
#[no_mangle]
pub unsafe extern "C" fn Java_com_example_android_MainActivity_getArray(
    env: JNIEnv,
    _: JObject,
    j_recipient: JString,
    jeopardy_mode: bool,
    question_num: i32,
) -> jarray {
    let array: jobjectArray = env
        .new_object_array(
            5,
            env.find_class("java/lang/String").unwrap(),
            *env.new_string("").unwrap(),
        )
        .unwrap();
    let mut i = 0;
    let mut this_string = "Feld";
    let this_second_string = CString::from(CStr::from_ptr(
        env.get_string(j_recipient).unwrap().as_ptr(),
    ))
    .to_str()
    .unwrap()
    .to_string();
    while (i < 5) {
        if (jeopardy_mode == false) {
            this_string = "false";
        }
        if (jeopardy_mode == true) {
            this_string = "true";
        }
        env.set_object_array_element(
            array,
            i,
            *env.new_string(
                this_second_string.to_owned() + &this_string.to_owned() + &i.to_string().as_str(),
            )
            .unwrap()
            .to_owned(),
        );
        i = i + 1;
    }
    return array;
}
*/
