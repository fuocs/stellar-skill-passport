#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Course {
    pub id: u32,
    pub title: String,
    pub verifier: Address,
    pub reward: u32,
    pub active: bool,
    pub completions: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Passport {
    pub learner: Address,
    pub points: u32,
    pub completed: u32,
    pub last_course: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Completion {
    pub learner: Address,
    pub course_id: u32,
    pub verifier: Address,
    pub evidence: String,
    pub reward: u32,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    NextCourseId,
    Course(u32),
    Completed(Address, u32),
    Points(Address),
    CompletedCount(Address),
    LastCourse(Address),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    CourseNotFound = 3,
    CourseInactive = 4,
    AlreadyCompleted = 5,
    RewardOverflow = 6,
}

#[contract]
pub struct SkillPassportContract;

#[contractimpl]
impl SkillPassportContract {
    pub fn init(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }

        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::NextCourseId, &1_u32);
        Ok(())
    }

    pub fn create_course(
        env: Env,
        title: String,
        verifier: Address,
        reward: u32,
    ) -> Result<u32, Error> {
        let admin = read_admin(&env)?;
        admin.require_auth();

        let id = read_next_course_id(&env)?;
        let course = Course {
            id,
            title,
            verifier,
            reward,
            active: true,
            completions: 0,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Course(id), &course);
        let next_id = match id.checked_add(1) {
            Some(value) => value,
            None => return Err(Error::RewardOverflow),
        };
        env.storage()
            .instance()
            .set(&DataKey::NextCourseId, &next_id);

        Ok(id)
    }

    pub fn set_course_active(env: Env, course_id: u32, active: bool) -> Result<Course, Error> {
        let admin = read_admin(&env)?;
        admin.require_auth();

        let mut course = read_course(&env, course_id)?;
        course.active = active;
        env.storage()
            .persistent()
            .set(&DataKey::Course(course_id), &course);
        Ok(course)
    }

    pub fn verify(
        env: Env,
        learner: Address,
        course_id: u32,
        evidence: String,
    ) -> Result<Completion, Error> {
        let mut course = read_course(&env, course_id)?;
        if !course.active {
            return Err(Error::CourseInactive);
        }
        if env
            .storage()
            .persistent()
            .has(&DataKey::Completed(learner.clone(), course_id))
        {
            return Err(Error::AlreadyCompleted);
        }

        course.verifier.require_auth();

        let points = read_u32(&env, DataKey::Points(learner.clone()));
        let next_points = match points.checked_add(course.reward) {
            Some(value) => value,
            None => return Err(Error::RewardOverflow),
        };
        let completed_count = match read_u32(&env, DataKey::CompletedCount(learner.clone()))
            .checked_add(1)
        {
            Some(value) => value,
            None => return Err(Error::RewardOverflow),
        };

        course.completions = match course.completions.checked_add(1) {
            Some(value) => value,
            None => return Err(Error::RewardOverflow),
        };
        let completion = Completion {
            learner: learner.clone(),
            course_id,
            verifier: course.verifier.clone(),
            evidence,
            reward: course.reward,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Course(course_id), &course);
        env.storage()
            .persistent()
            .set(&DataKey::Completed(learner.clone(), course_id), &completion);
        env.storage()
            .persistent()
            .set(&DataKey::Points(learner.clone()), &next_points);
        env.storage()
            .persistent()
            .set(&DataKey::CompletedCount(learner.clone()), &completed_count);
        env.storage()
            .persistent()
            .set(&DataKey::LastCourse(learner), &course_id);

        Ok(completion)
    }

    pub fn get_course(env: Env, course_id: u32) -> Result<Course, Error> {
        read_course(&env, course_id)
    }

    pub fn has_completed(env: Env, learner: Address, course_id: u32) -> bool {
        env.storage()
            .persistent()
            .has(&DataKey::Completed(learner, course_id))
    }

    pub fn get_completion(
        env: Env,
        learner: Address,
        course_id: u32,
    ) -> Result<Completion, Error> {
        match env
            .storage()
            .persistent()
            .get(&DataKey::Completed(learner, course_id))
        {
            Some(completion) => Ok(completion),
            None => Err(Error::CourseNotFound),
        }
    }

    pub fn passport(env: Env, learner: Address) -> Passport {
        Passport {
            learner: learner.clone(),
            points: read_u32(&env, DataKey::Points(learner.clone())),
            completed: read_u32(&env, DataKey::CompletedCount(learner.clone())),
            last_course: read_u32(&env, DataKey::LastCourse(learner)),
        }
    }
}

fn read_admin(env: &Env) -> Result<Address, Error> {
    let admin: Option<Address> = env.storage().instance().get(&DataKey::Admin);
    match admin {
        Some(admin) => Ok(admin),
        None => Err(Error::NotInitialized),
    }
}

fn read_next_course_id(env: &Env) -> Result<u32, Error> {
    match env.storage().instance().get(&DataKey::NextCourseId) {
        Some(id) => Ok(id),
        None => Err(Error::NotInitialized),
    }
}

fn read_course(env: &Env, course_id: u32) -> Result<Course, Error> {
    match env.storage().persistent().get(&DataKey::Course(course_id)) {
        Some(course) => Ok(course),
        None => Err(Error::CourseNotFound),
    }
}

fn read_u32(env: &Env, key: DataKey) -> u32 {
    env.storage().persistent().get(&key).unwrap_or(0)
}
