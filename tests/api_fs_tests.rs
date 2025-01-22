#[test]
fn api_fs_tests() {
    // 测试某个 API
    let result = some_api_function();
    assert_eq!(result, 42);
}

fn some_api_function() -> i32 {
    // 模拟 API 调用
    42
}