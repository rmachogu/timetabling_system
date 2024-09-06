#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use regex::Regex;
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// User Roles
#[derive(candid::CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
enum UserRole {
    #[default]
    Student,
    Instructor,
    Admin,
}

// Course struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Course {
    id: u64,
    name: String,
    duration_years: u32,
    required_equipment: String,
    prerequisites: Vec<u64>,
}

// Instructor struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Instructor {
    id: u64,
    name: String,
    availability: Vec<String>,
    preferred_times: Vec<String>,
}

// Classroom struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Classroom {
    id: u64,
    name: String,
    capacity: u32,
    equipment: String,
}

// Timetable struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Timetable {
    id: u64,
    course_id: u64,
    instructor_id: u64,
    classroom_id: u64,
    time_slot: String,
}

// User struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct User {
    id: u64,
    username: String,
    password: String,
    role: UserRole,
    email: String,
    created_at: u64,
}

// Implementing the Storable trait for the structs
impl Storable for Course {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Course {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Instructor {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Instructor {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Classroom {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Classroom {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Timetable {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Timetable {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for User {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for User {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Memory Storage
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static COURSE_STORAGE: RefCell<StableBTreeMap<u64, Course, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static INSTRUCTOR_STORAGE: RefCell<StableBTreeMap<u64, Instructor, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static CLASSROOM_STORAGE: RefCell<StableBTreeMap<u64, Classroom, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));

    static TIMETABLE_STORAGE: RefCell<StableBTreeMap<u64, Timetable, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    ));

    static USER_STORAGE: RefCell<StableBTreeMap<u64, User, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5)))
    ));
}

// Helper Functions
fn current_time() -> u64 {
    time()
}

fn validate_email(email: &str) -> Result<(), String> {
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    if !email_regex.is_match(email) {
        return Err("Invalid email address.".to_string());
    }
    Ok(())
}

fn validate_password(password: &str) -> Result<(), String> {
    let min_length = 8;
    if password.len() < min_length {
        return Err("Password must be at least 8 characters long.".to_string());
    }
    Ok(())
}

fn is_email_unique(email: &str) -> bool {
    USER_STORAGE.with(|storage| storage.borrow().iter().all(|(_, user)| user.email != email))
}

// User Management
#[derive(candid::CandidType, Deserialize, Serialize)]
struct UserPayload {
    username: String,
    password: String,
    email: String,
    role: UserRole,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct ChangeUserRolePayload {
    user_id: u64,
    role: UserRole,
}

#[ic_cdk::update]
fn create_user(payload: UserPayload) -> Result<User, String> {
    if payload.username.is_empty() || payload.password.is_empty() || payload.email.is_empty() {
        return Err("Ensure 'username', 'password', and 'email' are provided.".to_string());
    }
    validate_email(&payload.email)?;
    if !is_email_unique(&payload.email) {
        return Err("Email address already exists.".to_string());
    }
    validate_password(&payload.password)?;

    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    }).expect("Cannot increment ID counter");

    let user = User {
        id,
        username: payload.username,
        password: payload.password,
        email: payload.email,
        role: payload.role,
        created_at: current_time(),
    };
    USER_STORAGE.with(|storage| storage.borrow_mut().insert(id, user.clone()));
    Ok(user)
}

#[ic_cdk::query]
fn get_users() -> Result<Vec<User>, String> {
    USER_STORAGE.with(|storage| {
        let users: Vec<User> = storage.borrow().iter().map(|(_, user)| user.clone()).collect();
        if users.is_empty() {
            Err("No users found".to_string())
        } else {
            Ok(users)
        }
    })
}

#[ic_cdk::update]
fn change_user_role(payload: ChangeUserRolePayload) -> Result<User, String> {
    let user = USER_STORAGE.with(|storage| {
        storage.borrow().iter().find(|(_, user)| user.id == payload.user_id).map(|(_, user)| user.clone())
    }).ok_or("User not found".to_string())?;

    let updated_user = User {
        role: payload.role,
        ..user
    };
    USER_STORAGE.with(|storage| storage.borrow_mut().insert(user.id, updated_user.clone()));
    Ok(updated_user)
}

// Additional Utility Functions
#[ic_cdk::update]
fn delete_user(user_id: u64) -> Result<String, String> {
    USER_STORAGE.with(|storage| {
        if storage.borrow_mut().remove(&user_id).is_some() {
            Ok("User deleted successfully.".to_string())
        } else {
            Err("User not found.".to_string())
        }
    })
}

#[ic_cdk::update]
fn update_user(user_id: u64, payload: UserPayload) -> Result<User, String> {
    let mut user = USER_STORAGE.with(|storage| {
        storage.borrow().iter().find(|(_, user)| user.id == user_id).map(|(_, user)| user.clone())
    }).ok_or("User not found".to_string())?;

    // Update fields
    user.username = payload.username;
    user.password = payload.password;
    user.email = payload.email;
    user.role = payload.role;

    USER_STORAGE.with(|storage| storage.borrow_mut().insert(user_id, user.clone()));
    Ok(user)
}

// Course Management
#[derive(candid::CandidType, Deserialize, Serialize)]
struct CoursePayload {
    name: String,
    duration_years: u32,
    required_equipment: String,
    prerequisites: Vec<u64>,
}

#[ic_cdk::update]
fn create_course(payload: CoursePayload) -> Result<Course, String> {
    if payload.name.is_empty() || payload.duration_years == 0 {
        return Err("Ensure 'name' and 'duration_years' are provided.".to_string());
    }
    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    }).expect("Cannot increment ID counter");

    let course = Course {
        id,
        name: payload.name,
        duration_years: payload.duration_years,
        required_equipment: payload.required_equipment,
        prerequisites: payload.prerequisites,
    };
    COURSE_STORAGE.with(|storage| storage.borrow_mut().insert(id, course.clone()));
    Ok(course)
}

#[ic_cdk::query]
fn get_courses() -> Result<Vec<Course>, String> {
    COURSE_STORAGE.with(|storage| {
        let courses: Vec<Course> = storage.borrow().iter().map(|(_, course)| course.clone()).collect();
        if courses.is_empty() {
            Err("No courses found".to_string())
        } else {
            Ok(courses)
        }
    })
}

// Instructor Management
#[derive(candid::CandidType, Deserialize, Serialize)]
struct InstructorPayload {
    name: String,
    availability: Vec<String>,
    preferred_times: Vec<String>,
}

#[ic_cdk::update]
fn create_instructor(payload: InstructorPayload) -> Result<Instructor, String> {
    if payload.name.is_empty() || payload.availability.is_empty() {
        return Err("Ensure 'name' and 'availability' are provided.".to_string());
    }
    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    }).expect("Cannot increment ID counter");

    let instructor = Instructor {
        id,
        name: payload.name,
        availability: payload.availability,
        preferred_times: payload.preferred_times,
    };
    INSTRUCTOR_STORAGE.with(|storage| storage.borrow_mut().insert(id, instructor.clone()));
    Ok(instructor)
}

#[ic_cdk::query]
fn get_instructors() -> Result<Vec<Instructor>, String> {
    INSTRUCTOR_STORAGE.with(|storage| {
        let instructors: Vec<Instructor> = storage.borrow().iter().map(|(_, instructor)| instructor.clone()).collect();
        if instructors.is_empty() {
            Err("No instructors found".to_string())
        } else {
            Ok(instructors)
        }
    })
}

// Classroom Management
#[derive(candid::CandidType, Deserialize, Serialize)]
struct ClassroomPayload {
    name: String,
    capacity: u32,
    equipment: String,
}

#[ic_cdk::update]
fn create_classroom(payload: ClassroomPayload) -> Result<Classroom, String> {
    if payload.name.is_empty() || payload.capacity == 0 {
        return Err("Ensure 'name' and 'capacity' are provided.".to_string());
    }
    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    }).expect("Cannot increment ID counter");

    let classroom = Classroom {
        id,
        name: payload.name,
        capacity: payload.capacity,
        equipment: payload.equipment,
    };
    CLASSROOM_STORAGE.with(|storage| storage.borrow_mut().insert(id, classroom.clone()));
    Ok(classroom)
}

#[ic_cdk::query]
fn get_classrooms() -> Result<Vec<Classroom>, String> {
    CLASSROOM_STORAGE.with(|storage| {
        let classrooms: Vec<Classroom> = storage.borrow().iter().map(|(_, classroom)| classroom.clone()).collect();
        if classrooms.is_empty() {
            Err("No classrooms found".to_string())
        } else {
            Ok(classrooms)
        }
    })
}

// Timetable Management
#[derive(candid::CandidType, Deserialize, Serialize)]
struct TimetablePayload {
    course_id: u64,
    instructor_id: u64,
    classroom_id: u64,
    time_slot: String,
}

#[ic_cdk::update]
fn create_timetable(payload: TimetablePayload) -> Result<Timetable, String> {
    if payload.course_id == 0 || payload.instructor_id == 0 || payload.classroom_id == 0 || payload.time_slot.is_empty() {
        return Err("Ensure 'course_id', 'instructor_id', 'classroom_id', and 'time_slot' are provided.".to_string());
    }
    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    }).expect("Cannot increment ID counter");

    let timetable = Timetable {
        id,
        course_id: payload.course_id,
        instructor_id: payload.instructor_id,
        classroom_id: payload.classroom_id,
        time_slot: payload.time_slot,
    };
    TIMETABLE_STORAGE.with(|storage| storage.borrow_mut().insert(id, timetable.clone()));
    Ok(timetable)
}

#[ic_cdk::query]
fn get_timetables() -> Result<Vec<Timetable>, String> {
    TIMETABLE_STORAGE.with(|storage| {
        let timetables: Vec<Timetable> = storage.borrow().iter().map(|(_, timetable)| timetable.clone()).collect();
        if timetables.is_empty() {
            Err("No timetables found".to_string())
        } else {
            Ok(timetables)
        }
    })
}

// Timetable Generation and Conflict Detection (simplified example)
fn generate_timetable() -> Result<Vec<Timetable>, String> {
    // This is a simplified example. A more complex algorithm is needed for actual implementation.
    let mut generated_timetables = vec![];
    let courses = get_courses()?;
    let instructors = get_instructors()?;
    let classrooms = get_classrooms()?;

    for course in courses {
        for instructor in &instructors {
            for classroom in &classrooms {
                let timetable = Timetable {
                    id: ID_COUNTER.with(|counter| {
                        let current_value = *counter.borrow().get();
                        counter.borrow_mut().set(current_value + 1)
                    }).expect("Cannot increment ID counter"),
                    course_id: course.id,
                    instructor_id: instructor.id,
                    classroom_id: classroom.id,
                    time_slot: "08:00-10:00".to_string(), // Example time slot
                };
                generated_timetables.push(timetable);
            }
        }
    }

    Ok(generated_timetables)
}

#[ic_cdk::update]
fn create_auto_timetable() -> Result<Vec<Timetable>, String> {
    let timetables = generate_timetable()?;
    for timetable in timetables.iter() {
        TIMETABLE_STORAGE.with(|storage| storage.borrow_mut().insert(timetable.id, timetable.clone()));
    }
    Ok(timetables)
}

// Exporting candid interface
ic_cdk::export_candid!();
