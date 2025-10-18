#include "napi/native_api.h"
#include "rustlog.h"
#include <cstdio>
#include <string>

static napi_value Add(napi_env env, napi_callback_info info) {
    size_t argc = 2;
    napi_value args[2] = {nullptr};

    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);

    napi_valuetype valuetype0;
    napi_typeof(env, args[0], &valuetype0);

    napi_valuetype valuetype1;
    napi_typeof(env, args[1], &valuetype1);

    double value0;
    napi_get_value_double(env, args[0], &value0);

    double value1;
    napi_get_value_double(env, args[1], &value1);

    napi_value sum;
    napi_create_double(env, value0 + value1, &sum);

    printf("Hello world!!! \n");

    return sum;
}

static napi_value LogInit(napi_env env, napi_callback_info info) {
    size_t argc = 1;
    napi_value args[1];
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);

    if (argc < 1) {
        napi_throw_type_error(env, nullptr, "Expected 1 arguments (tag, msg)");
        return nullptr;
    }

    // --- 获取第一个参数 dir ---
    size_t dir_len = 0;
    napi_get_value_string_utf8(env, args[0], nullptr, 0, &dir_len);
    char *dir = (char *)malloc(dir_len + 1);
    napi_get_value_string_utf8(env, args[0], dir, dir_len + 1, &dir_len);
    log_init(nullptr, dir);

    free(dir);

    napi_value sum;
    napi_create_double(env, 1, &sum);
    return sum;
}

static napi_value LogMessage(napi_env env, napi_callback_info info) {
    size_t argc = 3;
    napi_value args[3];
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);

    if (argc < 3) {
        napi_throw_type_error(env, nullptr, "Expected 3 arguments: (level, tag, msg)");
        return nullptr;
    }

    // --- 解析第一个参数 level ---
    size_t level_len = 0;
    napi_get_value_string_utf8(env, args[0], nullptr, 0, &level_len);
    char *level = (char *)malloc(level_len + 1);
    napi_get_value_string_utf8(env, args[0], level, level_len + 1, &level_len);

    // --- 解析第二个参数 tag ---
    size_t tag_len = 0;
    napi_get_value_string_utf8(env, args[1], nullptr, 0, &tag_len);
    char *tag = (char *)malloc(tag_len + 1);
    napi_get_value_string_utf8(env, args[1], tag, tag_len + 1, &tag_len);

    // --- 解析第三个参数 msg ---
    size_t msg_len = 0;
    napi_get_value_string_utf8(env, args[2], nullptr, 0, &msg_len);
    char *msg = (char *)malloc(msg_len + 1);
    napi_get_value_string_utf8(env, args[2], msg, msg_len + 1, &msg_len);

    // --- 打印日志（这里可以替换为系统日志接口） ---
    printf("[%s][%s] %s\n", level, tag, msg);
    log_write(level, tag, msg);

    // --- 释放内存 ---
    free(level);
    free(tag);
    free(msg);


//    std::string level = "info";
//    std::string tag = "RustLogNapi";
//    std::string msg = "Hello world from napi 123";
//    log_write(level.c_str(), tag.c_str(), msg.c_str());

    napi_value sum;
    napi_create_double(env, 1, &sum);
    return sum;
}

EXTERN_C_START
static napi_value Init(napi_env env, napi_value exports) {
    napi_property_descriptor desc[] = {
        {"add", nullptr, Add, nullptr, nullptr, nullptr, napi_default, nullptr},
        {"logInit", nullptr, LogInit, nullptr, nullptr, nullptr, napi_default, nullptr},
        {"logMessage", nullptr, LogMessage, nullptr, nullptr, nullptr, napi_default, nullptr},
    };
    napi_define_properties(env, exports, sizeof(desc) / sizeof(desc[0]), desc);
    return exports;
}
EXTERN_C_END

static napi_module demoModule = {
    .nm_version = 1,
    .nm_flags = 0,
    .nm_filename = nullptr,
    .nm_register_func = Init,
    .nm_modname = "entry",
    .nm_priv = ((void *)0),
    .reserved = {0},
};

extern "C" __attribute__((constructor)) void RegisterEntryModule(void) { napi_module_register(&demoModule); }
