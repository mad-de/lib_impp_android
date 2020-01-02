#[allow(dead_code)]
mod lib_impp;

use jni::objects::{JObject, JString};
use jni::sys::{jarray, jobjectArray, jstring};
use jni::JNIEnv;
use std::convert::TryFrom;
use std::ffi::{CStr, CString};

pub static FILES_PATH: &str = "/data/user/0/com.example.android/files/";

// Return page Title
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

// Return true when database is built
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

// Return true when database exists
#[no_mangle]
pub unsafe extern "C" fn Java_com_example_android_MainActivity_getDatabaseStatus(
    _env: JNIEnv,
    _: JObject,
    _j_recipient: JString,
) -> bool {
    lib_impp::get_database_status(&FILES_PATH)
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
    jeopardy_mode: bool,
) -> jarray {
    let question_details_array: [String; 4] =
        lib_impp::get_question_details(question_num, jeopardy_mode, &FILES_PATH);
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

// Return Array from Vector with Multiple-Choice Distractors
#[no_mangle]
pub unsafe extern "C" fn Java_com_example_android_MainActivity_getMCDistractors(
    env: JNIEnv,
    _: JObject,
    question_num: i32,
    distractor_amount: i32,
    jeopardy_mode: bool,
) -> jarray {
    let question_details_vec =
        lib_impp::get_mc_distractors(question_num, distractor_amount, jeopardy_mode, &FILES_PATH);
    // Initialize our array with the length of the vector
    let array: jobjectArray = env
        .new_object_array(
            i32::try_from(question_details_vec.len()).unwrap(),
            env.find_class("java/lang/String").unwrap(),
            *env.new_string("").unwrap(),
        )
        .unwrap();
    let mut i = 0;
    while i < i32::try_from(question_details_vec.len()).unwrap() {
        // Edit every Item of the Array to give it the values we want
        env.set_object_array_element(
            array,
            i,
            *env.new_string(
                question_details_vec
                    [usize::try_from(i).expect("Variable i could not be converted to usize.")]
                .answer
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

// Return Array from HashMap with all categories
#[no_mangle]
pub unsafe extern "C" fn Java_com_example_android_MainActivity_getCategories(
    env: JNIEnv,
    _: JObject,
    _j_recipient: JString,
) -> jarray {
    let categories = lib_impp::get_categories(&FILES_PATH);
    // Initialize our array with the length of the vector
    let array: jobjectArray = env
        .new_object_array(
            i32::try_from(categories.len()).unwrap(),
            env.find_class("java/lang/String").unwrap(),
            *env.new_string("").unwrap(),
        )
        .unwrap();
    let mut i = 0;
    // Edit every Item of the Array to give it the values we want
    for item in &categories {
        env.set_object_array_element(
            array,
            i,
            *env.new_string(item.to_owned()).unwrap().to_owned(),
        )
        .expect("Could not perform set_object_array_element on array element.");
        i += 1;
    }
    array
}

/*
// Return Arraylist - TODO - SIGABRT 6
#[no_mangle]
pub unsafe extern "C" fn Java_com_example_android_MainActivity_getArraylist(
    env: JNIEnv,
    _: JObject,
    _question_num: i32
) -> jobject {
    let cls_arraylist = env.find_class("java/util/ArrayList").unwrap();
    let arraylist = env.new_object(cls_arraylist, "()V", &[]).unwrap();
    let mut i = 0;
    while i < 7 {
        // Add items
        env.call_method(
            arraylist,
            "add",
            "(Ljava/lang/Object;)Z",
            &[JValue::from(JObject::from(env.new_string("PETER".to_string()).unwrap()))],
        )
        .unwrap();
        i += 1;
    }
    *arraylist
}
*/
