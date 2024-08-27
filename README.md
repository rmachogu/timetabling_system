
# Timetabling System on Internet Computer Protocol (ICP)

This project is a decentralized platform built on the Internet Computer Protocol (ICP) for automating and optimizing the creation of timetables for educational institutions. It allows administrators to manage courses, instructors, classrooms, and constraints while generating conflict-free timetables based on predefined preferences and constraints.

## Key Features

### User Management
- **Create User**: Allows administrators to create user profiles.
- **Update User Role**: Allows administrators to update user roles (Student, Instructor, Admin).
- **Get All Users**: Retrieve a list of all users in the system.

### Course Management
- **Add Course**: Allows administrators to add courses to the system.
- **Update Course**: Allows administrators to update existing course information.
- **Get All Courses**: Retrieve a list of all courses in the system.

### Instructor Management
- **Add Instructor**: Allows administrators to add instructor profiles.
- **Update Instructor**: Allows administrators to update instructor availability and preferences.
- **Get All Instructors**: Retrieve a list of all instructors in the system.

### Classroom Management
- **Add Classroom**: Allows administrators to add classrooms to the system.
- **Update Classroom**: Allows administrators to update classroom information.
- **Get All Classrooms**: Retrieve a list of all classrooms in the system.

### Timetable Management
- **Create Timetable**: Allows administrators to create timetables for courses.
- **Generate Automatic Timetable**: Automatically generates conflict-free timetables based on input data.
- **Get All Timetables**: Retrieve a list of all timetables.

## Error Handling
- **Not Found**: Returns an error if a requested item is not found.
- **Unauthorized Access**: Returns an error if a user tries to perform an action without necessary permissions.

## Sample Payloads

### UserPayload
```json
{
  "username": "jane.doe",
  "password": "SecureP@ssword123",
  "email": "jane.doe@example.com",
  "role": "Admin"
}
```

### CoursePayload
```json
{
  "name": "Introduction to Computer Science",
  "duration": 60,
  "required_equipment": "Projector, Whiteboard",
  "prerequisites": [1, 2]
}
```

### InstructorPayload
```json
{
  "name": "Dr. John Smith",
  "availability": ["Monday 08:00-10:00", "Wednesday 10:00-12:00"],
  "preferred_times": ["Monday 08:00-10:00"]
}
```

### ClassroomPayload
```json
{
  "name": "Room 101",
  "capacity": 30,
  "equipment": "Projector, Whiteboard"
}
```

### TimetablePayload
```json
{
  "course_id": 1,
  "instructor_id": 1,
  "classroom_id": 1,
  "time_slot": "Monday 08:00-10:00"
}
```


## Requirements

- rustc 1.64 or higher

```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ source "$HOME/.cargo/env"
```

- rust wasm32-unknown-unknown target

```bash
$ rustup target add wasm32-unknown-unknown
```

- candid-extractor

```bash
$ cargo install candid-extractor
```

- install `dfx`

```bash
$ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
$ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
$ source ~/.bashrc
$ dfx start --background
```

If you want to start working on your project right away, you might want to try the following commands:

```bash
$ cd icp_rust_boilerplate/
$ dfx help
$ dfx canister --help
```

## Update dependencies

update the `dependencies` block in `/src/{canister_name}/Cargo.toml`:

```
[dependencies]
candid = "0.9.9"
ic-cdk = "0.11.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
ic-stable-structures = { git = "https://github.com/lwshang/stable-structures.git", branch = "lwshang/update_cdk"}
```

## did autogenerate

Add this script to the root directory of the project:

```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh
```

Update line 16 with the name of your canister:

```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh#L16
```

After this run this script to generate Candid.
Important note!

You should run this script each time you modify/add/remove exported functions of the canister.
Otherwise, you'll have to modify the candid file manually.

Also, you can add package json with this content:

```
{
    "scripts": {
        "generate": "./did.sh && dfx generate",
        "gen-deploy": "./did.sh && dfx generate && dfx deploy -y"
      }
}
```

and use commands `npm run generate` to generate candid or `npm run gen-deploy` to generate candid and to deploy a canister.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
$ dfx start --background

# Deploys your canisters to the replica and generates your candid interface
$ dfx deploy
```
