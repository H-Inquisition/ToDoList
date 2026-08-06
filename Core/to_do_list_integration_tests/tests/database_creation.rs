use core::app_state::AppState;
use core::common::{Priority, Status, Task};
use integration_tests::constants;
use integration_tests::helpers;
use integration_tests::helpers::expected_sets;

#[test]
fn create_default_database_set_a_delete_update() {
    let mut app_state =
        AppState::new(constants::DATABASE_SET_A_FILENAME).unwrap();

    let second_expected_task_set = expected_sets::dataset_a_first_modification();
    let third_expected_task_set = expected_sets::dataset_a_second_modification();

    app_state
        .add_task("test_task_one".to_string(), Priority::Medium)
        .unwrap();
    app_state
        .add_task("test_task_two".to_string(), Priority::High)
        .unwrap();
    app_state
        .add_task("test_task_three".to_string(), Priority::Medium)
        .unwrap();
    app_state
        .add_task("test_task_four".to_string(), Priority::Low)
        .unwrap();
    app_state
        .add_task("test_task_five".to_string(), Priority::Medium)
        .unwrap();

    let actual_tasks_result = app_state.get_tasks();
    assert!(actual_tasks_result.is_ok());
    assert_eq!(actual_tasks_result.unwrap(), expected_sets::basic_expected_task_set());

    app_state.delete_task(3).unwrap();

    let actual_tasks_result = app_state.get_tasks();
    assert!(actual_tasks_result.is_ok());
    assert_eq!(actual_tasks_result.unwrap(), second_expected_task_set);

    app_state.update_task(2, Task {
        status: Status::Done,
        title: "test_task_modified".to_string(),
        priority: Priority::Low,
    }).unwrap();

    let actual_tasks_result = app_state.get_tasks();
    assert!(actual_tasks_result.is_ok());
    assert_eq!(actual_tasks_result.unwrap(), third_expected_task_set);
    helpers::cleanup(constants::DATABASE_SET_A_FILENAME);
}

#[test]
fn create_default_set_b_delete_update_multiple() {
    let mut app_state =
        AppState::new(constants::DATABASE_SET_B_FILENAME).unwrap();

    let second_expected_task_set = expected_sets::dataset_b_first_modification();
    let third_expected_task_set = expected_sets::dataset_b_second_modification();

    app_state
        .add_task("test_task_one".to_string(), Priority::Medium)
        .unwrap();
    app_state
        .add_task("test_task_two".to_string(), Priority::High)
        .unwrap();
    app_state
        .add_task("test_task_three".to_string(), Priority::Medium)
        .unwrap();
    app_state
        .add_task("test_task_four".to_string(), Priority::Low)
        .unwrap();
    app_state
        .add_task("test_task_five".to_string(), Priority::Medium)
        .unwrap();

    let actual_tasks_result = app_state.get_tasks();
    assert!(actual_tasks_result.is_ok());
    assert_eq!(actual_tasks_result.unwrap(), expected_sets::basic_expected_task_set());

    app_state.delete_task(3).unwrap();
    app_state.delete_task(4).unwrap();

    let actual_tasks_result = app_state.get_tasks();
    assert!(actual_tasks_result.is_ok());
    assert_eq!(actual_tasks_result.unwrap(), second_expected_task_set);

    app_state
        .add_task("test_task_six".to_string(), Priority::High)
        .unwrap();
    app_state
        .add_task("test_task_seven".to_string(), Priority::Medium)
        .unwrap();
    app_state
        .add_task("test_task_eight".to_string(), Priority::Low)
        .unwrap();
    app_state
        .add_task("test_task_nine".to_string(), Priority::Medium)
        .unwrap();

    let actual_tasks_result = app_state.get_tasks();
    assert!(actual_tasks_result.is_ok());
    assert_eq!(actual_tasks_result.unwrap(), third_expected_task_set);
    helpers::cleanup(constants::DATABASE_SET_B_FILENAME);
}

#[test]
fn create_database_set_c() {
    let mut app_state =
        AppState::new(constants::DATABASE_SET_C_FILENAME).unwrap();

    let second_expected_task_set = expected_sets::dataset_c_first_modification();
    let third_expected_task_set = expected_sets::dataset_c_second_modification();

    app_state
        .add_task("test_task_one".to_string(), Priority::Medium)
        .unwrap();
    app_state
        .add_task("test_task_two".to_string(), Priority::High)
        .unwrap();
    app_state
        .add_task("test_task_three".to_string(), Priority::Medium)
        .unwrap();
    app_state
        .add_task("test_task_four".to_string(), Priority::Low)
        .unwrap();
    app_state
        .add_task("test_task_five".to_string(), Priority::Medium)
        .unwrap();

    let actual_tasks_result = app_state.get_tasks();
    assert!(actual_tasks_result.is_ok());
    assert_eq!(actual_tasks_result.unwrap(), expected_sets::basic_expected_task_set());

    app_state.delete_task(4).unwrap();

    let actual_tasks_result = app_state.get_tasks();
    assert!(actual_tasks_result.is_ok());
    assert_eq!(actual_tasks_result.unwrap(), second_expected_task_set);

    app_state.update_task(3, Task {
        status: Status::Done,
        title: "test_task_modified".to_string(),
        priority: Priority::High,
    }).unwrap();
    app_state
        .add_task("test_task_six".to_string(), Priority::Medium)
        .unwrap();
    app_state
        .add_task("test_task_seven".to_string(), Priority::High)
        .unwrap();
    app_state
        .add_task("test_task_eight".to_string(), Priority::High)
        .unwrap();

    let actual_tasks_result = app_state.get_tasks();
    assert!(actual_tasks_result.is_ok());
    assert_eq!(actual_tasks_result.unwrap(), third_expected_task_set);
    helpers::cleanup(constants::DATABASE_SET_C_FILENAME);
}
