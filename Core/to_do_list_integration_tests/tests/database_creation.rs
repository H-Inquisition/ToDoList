use std::path::Path;
use core::app_state::AppState;
use core::common::{Priority, Status, Task};
use integration_tests::constants;


#[test]
fn create_database_set_a() {
    cleanup(constants::DATABASE_SET_A_FILENAME);
    let mut app_state =
        AppState::new(constants::TEST_PORT, constants::DATABASE_SET_A_FILENAME).unwrap();

    let second_expected_task_set = vec![
        (
            1,
            Task {
                status: Status::Planned,
                title: "test_task_one".to_string(),
                priority: Priority::Medium,
            },
        ),
        (
            2,
            Task {
                status: Status::Planned,
                title: "test_task_two".to_string(),
                priority: Priority::High,
            },
        ),
        (
            4,
            Task {
                status: Status::Planned,
                title: "test_task_four".to_string(),
                priority: Priority::Low,
            },
        ),
        (
            5,
            Task {
                status: Status::Planned,
                title: "test_task_five".to_string(),
                priority: Priority::Medium,
            },
        ),
    ];
    let third_expected_task_set = vec![
        (
            1,
            Task {
                status: Status::Planned,
                title: "test_task_one".to_string(),
                priority: Priority::Medium,
            },
        ),
        (
            2,
            Task {
                status: Status::Done,
                title: "test_task_modified".to_string(),
                priority: Priority::Low,
            },
        ),
        (
            4,
            Task {
                status: Status::Planned,
                title: "test_task_four".to_string(),
                priority: Priority::Low,
            },
        ),
        (
            5,
            Task {
                status: Status::Planned,
                title: "test_task_five".to_string(),
                priority: Priority::Medium,
            },
        ),
    ];

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

    let actual_tasks_result = app_state.get_tasks_as_vector();
    assert!(actual_tasks_result.is_ok());
    assert_eq!(actual_tasks_result.unwrap(), basic_expected_task_set());

    app_state.delete_task(3).unwrap();

    let actual_tasks_result = app_state.get_tasks_as_vector();
    assert!(actual_tasks_result.is_ok());
    assert_eq!(actual_tasks_result.unwrap(), second_expected_task_set);

    app_state.update_task(2, Task {
        status: Status::Done,
        title: "test_task_modified".to_string(),
        priority: Priority::Low,
    }).unwrap();

    let actual_tasks_result = app_state.get_tasks_as_vector();
    assert!(actual_tasks_result.is_ok());
    assert_eq!(actual_tasks_result.unwrap(), third_expected_task_set);
}

#[test]
fn create_database_set_b() {
    cleanup(constants::DATABASE_SET_B_FILENAME);
    let mut app_state =
        AppState::new(constants::TEST_PORT, constants::DATABASE_SET_B_FILENAME).unwrap();

    let second_expected_task_set = vec![
        (
            1,
            Task {
                status: Status::Planned,
                title: "test_task_one".to_string(),
                priority: Priority::Medium,
            },
        ),
        (
            2,
            Task {
                status: Status::Planned,
                title: "test_task_two".to_string(),
                priority: Priority::High,
            },
        ),
        (
            5,
            Task {
                status: Status::Planned,
                title: "test_task_five".to_string(),
                priority: Priority::Medium,
            },
        ),
    ];
    let third_expected_task_set = vec![
        (
            1,
            Task {
                status: Status::Planned,
                title: "test_task_one".to_string(),
                priority: Priority::Medium,
            },
        ),
        (
            2,
            Task {
                status: Status::Planned,
                title: "test_task_two".to_string(),
                priority: Priority::High,
            },
        ),
        (
            5,
            Task {
                status: Status::Planned,
                title: "test_task_five".to_string(),
                priority: Priority::Medium,
            },
        ),
        (
            6,
            Task {
                status: Status::Planned,
                title: "test_task_six".to_string(),
                priority: Priority::High,
            },
        ),
        (
            7,
            Task {
                status: Status::Planned,
                title: "test_task_seven".to_string(),
                priority: Priority::Medium,
            },
        ),
        (
            8,
            Task {
                status: Status::Planned,
                title: "test_task_eight".to_string(),
                priority: Priority::Low,
            },
        ),
        (
            9,
            Task {
                status: Status::Planned,
                title: "test_task_nine".to_string(),
                priority: Priority::Medium,
            },
        ),
    ];

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

    let actual_tasks_result = app_state.get_tasks_as_vector();
    assert!(actual_tasks_result.is_ok());
    assert_eq!(actual_tasks_result.unwrap(), basic_expected_task_set());

    app_state.delete_task(3).unwrap();
    app_state.delete_task(4).unwrap();

    let actual_tasks_result = app_state.get_tasks_as_vector();
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

    let actual_tasks_result = app_state.get_tasks_as_vector();
    assert!(actual_tasks_result.is_ok());
    assert_eq!(actual_tasks_result.unwrap(), third_expected_task_set);
}

#[test]
fn create_database_set_c() {
    cleanup(constants::DATABASE_SET_C_FILENAME);
    let mut app_state =
        AppState::new(constants::TEST_PORT, constants::DATABASE_SET_C_FILENAME).unwrap();

    let second_expected_task_set = vec![
        (
            1,
            Task {
                status: Status::Planned,
                title: "test_task_one".to_string(),
                priority: Priority::Medium,
            },
        ),
        (
            2,
            Task {
                status: Status::Planned,
                title: "test_task_two".to_string(),
                priority: Priority::High,
            },
        ),
        (
            3,
            Task {
                status: Status::Planned,
                title: "test_task_three".to_string(),
                priority: Priority::Medium,
            },
        ),
        (
            5,
            Task {
                status: Status::Planned,
                title: "test_task_five".to_string(),
                priority: Priority::Medium,
            },
        ),
    ];
    let third_expected_task_set = vec![
        (
            1,
            Task {
                status: Status::Planned,
                title: "test_task_one".to_string(),
                priority: Priority::Medium,
            },
        ),
        (
            2,
            Task {
                status: Status::Planned,
                title: "test_task_two".to_string(),
                priority: Priority::High,
            },
        ),
        (
            3,
            Task {
                status: Status::Done,
                title: "test_task_modified".to_string(),
                priority: Priority::High,
            },
        ),
        (
            5,
            Task {
                status: Status::Planned,
                title: "test_task_five".to_string(),
                priority: Priority::Medium,
            },
        ),
        (
            6,
            Task {
                status: Status::Planned,
                title: "test_task_six".to_string(),
                priority: Priority::Medium,
            },
        ),
        (
            7,
            Task {
                status: Status::Planned,
                title: "test_task_seven".to_string(),
                priority: Priority::High,
            },
        ),
        (
            8,
            Task {
                status: Status::Planned,
                title: "test_task_eight".to_string(),
                priority: Priority::High,
            },
        ),
    ];

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

    let actual_tasks_result = app_state.get_tasks_as_vector();
    assert!(actual_tasks_result.is_ok());
    assert_eq!(actual_tasks_result.unwrap(), basic_expected_task_set());

    app_state.delete_task(4).unwrap();

    let actual_tasks_result = app_state.get_tasks_as_vector();
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

    let actual_tasks_result = app_state.get_tasks_as_vector();
    assert!(actual_tasks_result.is_ok());
    assert_eq!(actual_tasks_result.unwrap(), third_expected_task_set);
}

fn basic_expected_task_set() -> Vec<(i64, Task)> {
    vec![
        (
            1,
            Task {
                status: Status::Planned,
                title: "test_task_one".to_string(),
                priority: Priority::Medium,
            },
        ),
        (
            2,
            Task {
                status: Status::Planned,
                title: "test_task_two".to_string(),
                priority: Priority::High,
            },
        ),
        (
            3,
            Task {
                status: Status::Planned,
                title: "test_task_three".to_string(),
                priority: Priority::Medium,
            },
        ),
        (
            4,
            Task {
                status: Status::Planned,
                title: "test_task_four".to_string(),
                priority: Priority::Low,
            },
        ),
        (
            5,
            Task {
                status: Status::Planned,
                title: "test_task_five".to_string(),
                priority: Priority::Medium,
            },
        ),
    ]
}

fn cleanup(database_path: &str) {
    if Path::new(database_path).exists() {
        std::fs::remove_file(database_path).unwrap();
    }
}
