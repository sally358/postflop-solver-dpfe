// yes, we are writing another eval function because it uses a different output formal

use std::result;

pub(crate) struct MarkedHand
{
    base_hand: [usize; 2],
    board: Vec<usize>
}

fn get_full_hand(hand: &MarkedHand) -> Vec<usize>
{
    let mut unfilled: Vec<usize> = vec![];

    unfilled.push(hand.base_hand[0]);
    unfilled.push(hand.base_hand[1]);

    let mut board_but_better = hand.board.clone();

    unfilled.append(&mut board_but_better);
    
    unfilled.sort();

    unfilled
}

fn derank(cards: &Vec<usize>) -> Vec<usize>
{
    let mut deranked: Vec<usize> = vec![];

    for card in cards
    {
        deranked.push(card >> 2);
    }

    deranked
}

fn straight_data(hand: &MarkedHand) -> (i8, u8)
{
    let full_hand = get_full_hand(hand);

    let full_hand_deranked = derank(&full_hand);
    let board_deranked = derank(&hand.board);

    let mut result: (i8, u8) = (0, 0);

    let mut straight_counter: i8 = 0;
    let mut straight_height: usize = 0;

    // to wheel some wheels
    if (full_hand_deranked[full_hand_deranked.len() - 1] == 12) && (full_hand_deranked[0] == 0)
    {
        straight_counter = 1;
    }

    for i in 1..full_hand_deranked.len()
    {
        if full_hand_deranked[i] - full_hand_deranked[i-1] == 1
        {
            straight_counter += 1;
            straight_height = full_hand_deranked[i];
        }
        else if full_hand_deranked[i] == full_hand_deranked[i-1]
        {
            continue;
        }
        else
        {
            straight_counter = 0;
        }
    }

    if straight_counter < 5
    {
        result.0 = -1;
        result.1 = 0;

        return result;
    }
    else 
    {
        result.0 = straight_height as i8;
    }

    // check for other possible more nutted straights
    let mut straight_counter_2: i8 = 0;
    let mut straight_height_2: usize = 0;

    let mut straight_1: usize = 0;
    let mut straight_2: usize = 0;

    let mut gap_data: Vec<i8> = vec![];

    if (full_hand_deranked[full_hand_deranked.len() - 1] == 12) && (full_hand_deranked[0] == 0)
    {
        straight_counter_2 = 1;
        gap_data.push(0);
    }

    for i in 1..board_deranked.len()
    {
        // difference stuff
        let difference = board_deranked[i] - board_deranked[i-1];
        if gap_data.len() == 5
        {
            gap_data.remove(0);
        }
        gap_data.push(difference as i8);


        // board measures
        if board_deranked[i] - board_deranked[i-1] == 1
        {
            straight_counter_2 += 1;
            straight_height_2 = board_deranked[i];
        }
        else if full_hand_deranked[i] == full_hand_deranked[i-1]
        {
            continue;
        }
        else
        {
            let mut sum: i8 = gap_data.iter().sum();
            
            while sum > 2
            {
                sum -= gap_data[0];
                gap_data.remove(0);

                straight_counter_2 -= 1;
            }
        }

        if straight_counter_2 >= 5
        {
            straight_2 = straight_1;
            straight_1 = straight_height_2;
        }
    }

    let sum: i8 = gap_data.iter().sum();

    // extending upwards if possible
    if straight_counter_2 >= 3 && sum == 0 && straight_height_2 < 11
    {
        straight_2 = straight_1;
        straight_1 = straight_height_2 + 2;
    }
    else if straight_counter_2 >= 4 && sum < 2 && straight_height_2 < 12
    {
        straight_2 = straight_1;
        straight_1 = straight_height_2 + 1;
    }

    // or broadway
    else if straight_counter_2 == 3 && sum <= 1 && straight_height_2 == 11
    {
        straight_2 = straight_1;
        straight_1 = 12;
    }


    // returning shit

    if straight_height == straight_1
    {
        result.1 = 2;
    }
    else if straight_height_2 == straight_2
    {
        result.1 = 1;
    }
    else
    {
        result.1 = 0;
    }

    result
}