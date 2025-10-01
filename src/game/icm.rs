use crate::TreeConfig;

/// Evaluates one step of equities of each player in ICM$ and recursively calls evaluation of the next steps
fn run_tournament_equity(players: &Vec<(usize, i32)>, payouts: &Vec<f64>, equity_vector: &mut Vec<f64>, fraction: f64)
{
    let all_chips: i32 = untuple(players).iter().sum();

    if payouts.len() > 0
    {
        let mut payouts_local = payouts.clone();
        let current_payout = payouts_local.remove(0);

        for i in 0..players.len()
        {
            let victor_data = players[i];

            let probability: f64 = victor_data.1 as f64 / all_chips as f64;

            (*equity_vector)[victor_data.0] += current_payout as f64 * probability * fraction;

            let mut new_players = players.clone();
            new_players.remove(i);

            run_tournament_equity(&new_players, &payouts_local, equity_vector, fraction*probability);
        }
    }
}



/// Returns a tuple of oop and ip ICM equity changes
pub fn get_changed_value(tree_config: &TreeConfig, oop_exit: i32, ip_exit: i32) -> (f64, f64)
{
    let mut init_player_stacks = tree_config.icm_stacks.clone();
    init_player_stacks.push(tree_config.icm_stack_oop);
    init_player_stacks.push(tree_config.icm_stack_ip);
    let init_stacks_prepared = idficate(init_player_stacks);

    println!("Init stacks prepared: {:?}", init_stacks_prepared);

    let mut new_player_stacks = tree_config.icm_stacks.clone();
    new_player_stacks.push(oop_exit);
    new_player_stacks.push(ip_exit);
    let new_stacks_prepared = idficate(new_player_stacks);

    println!("New stacks prepared: {:?}", new_stacks_prepared);

    let payouts = tree_config.icm_payouts.clone();

    let mut init_tournament_equity: Vec<f64> = vec![0.0; init_stacks_prepared.len()];
    let mut new_tournament_equity: Vec<f64> = vec![0.0; new_stacks_prepared.len()];

    run_tournament_equity(&init_stacks_prepared, &payouts, &mut init_tournament_equity, 1.0);
    run_tournament_equity(&new_stacks_prepared, &payouts, &mut new_tournament_equity, 1.0);

    println!("Init tournament_equity: {:?}", init_tournament_equity);
    println!("New tournament_equity: {:?}", new_tournament_equity);

    let oop_init_equity = init_tournament_equity[init_tournament_equity.len() - 2];
    let oop_new_equity = new_tournament_equity[new_tournament_equity.len() - 2];

    let ip_init_equity = init_tournament_equity[init_tournament_equity.len() - 1];
    let ip_new_equity = new_tournament_equity[new_tournament_equity.len() - 1];

    let oop_change = oop_new_equity - oop_init_equity;
    let ip_change = ip_new_equity - ip_init_equity;

    (oop_change, ip_change)
}



/// Adds id values to each stack (necessary to correctly update tournament equity vector)
fn idficate(stacks_init: Vec<i32>) -> Vec<(usize, i32)>
{
    let mut stacks_with_ids: Vec<(usize, i32)> = vec![];

    for i in 0..stacks_init.len()
    {
        stacks_with_ids.push((i, stacks_init[i]));
    }

    stacks_with_ids
}



/// Turns a vector of (id, value) tuples into just a vector of values
fn untuple(vector: &Vec<(usize, i32)>) -> Vec<i32>
{
    let mut value_vector: Vec<i32> = vec![0; vector.len()];

    for i in 0..value_vector.len()
    {
        value_vector[i] = vector[i].1;
    }

    value_vector

}
