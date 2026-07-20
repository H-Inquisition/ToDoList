use core::common::{Priority, Task, Status};

pub fn basic_expected_task_set() -> Vec<(i64, Task)> {
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

pub fn dataset_a_first_modification() -> Vec<(i64, Task)> {
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
pub fn dataset_a_second_modification() -> Vec<(i64, Task)> {
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
    ]
}

pub fn dataset_b_first_modification() -> Vec<(i64, Task)> {
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
            5,
            Task {
                status: Status::Planned,
                title: "test_task_five".to_string(),
                priority: Priority::Medium,
            },
        ),
    ]
}
pub fn dataset_b_second_modification() -> Vec<(i64, Task)> {
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
    ]
}

pub fn dataset_c_first_modification() -> Vec<(i64, Task)> {
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
            5,
            Task {
                status: Status::Planned,
                title: "test_task_five".to_string(),
                priority: Priority::Medium,
            },
        ),
    ]
}
pub fn dataset_c_second_modification() -> Vec<(i64, Task)> {
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
    ]
}