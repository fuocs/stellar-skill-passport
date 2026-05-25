use super::*;
use soroban_sdk::{testutils::Address as _, Env, String};

#[test]
fn verifier_awards_learning_passport_points() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(SkillPassportContract, ());
    let client = SkillPassportContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let verifier = Address::generate(&env);
    let learner = Address::generate(&env);

    assert_eq!(client.init(&admin), Ok(()));

    let course_id = client.create_course(
        &String::from_str(&env, "Soroban Smart Contract Basics"),
        &verifier,
        &75,
    )
    .unwrap();

    let completion = client.verify(
        &learner,
        &course_id,
        &String::from_str(&env, "https://github.com/student/stellar-project"),
    )
    .unwrap();

    assert_eq!(completion.reward, 75);
    assert_eq!(completion.verifier, verifier);
    assert!(client.has_completed(&learner, &course_id));

    let passport = client.passport(&learner);
    assert_eq!(passport.points, 75);
    assert_eq!(passport.completed, 1);
    assert_eq!(passport.last_course, course_id);
}

#[test]
fn learner_cannot_receive_duplicate_credit() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(SkillPassportContract, ());
    let client = SkillPassportContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let verifier = Address::generate(&env);
    let learner = Address::generate(&env);

    assert_eq!(client.init(&admin), Ok(()));
    let course_id = client
        .create_course(&String::from_str(&env, "Freighter Wallet"), &verifier, &20)
        .unwrap();

    client
        .verify(&learner, &course_id, &String::from_str(&env, "first proof"))
        .unwrap();
    assert_eq!(
        client.verify(&learner, &course_id, &String::from_str(&env, "duplicate proof")),
        Err(Error::AlreadyCompleted)
    );
}
