use super::*;
use crate::interface::*;
use std::ptr;
use std::slice;


impl GameNode for PostFlopNode {
    type P = PostFlopPair;

    #[inline]
    fn is_terminal(&self) -> bool {
        self.player & PLAYER_TERMINAL_FLAG != 0
    }

    #[inline]
    fn is_chance(&self) -> bool {
        self.player & PLAYER_CHANCE_FLAG != 0
    }

    #[inline]
    fn cfvalue_storage_player(&self) -> Option<usize> {
        let prev_player = self.player & PLAYER_MASK;
        match prev_player {
            0 => Some(1),
            1 => Some(0),
            _ => None,
        }
    }

    #[inline]
    fn player(&self) -> usize {
        self.player as usize
    }

    #[inline]
    fn num_actions(&self) -> usize {
        self.num_children as usize
    }

    #[inline]
    fn play(&self, action: usize) -> MutexGuardLike<Self> {
        self.children()[action].lock()
    }

    #[inline]
    fn strategy(&self) -> &[f32] {
        unsafe { slice::from_raw_parts(self.storage1 as *const f32, self.num_elements as usize) }
    }

    #[inline]
    fn strategy_mut(&mut self) -> &mut [f32] {
        unsafe { slice::from_raw_parts_mut(self.storage1 as *mut f32, self.num_elements as usize) }
    }

    #[inline]
    fn regrets(&self) -> &[f32] {
        unsafe { slice::from_raw_parts(self.storage2 as *const f32, self.num_elements as usize) }
    }

    #[inline]
    fn regrets_mut(&mut self) -> &mut [f32] {
        unsafe { slice::from_raw_parts_mut(self.storage2 as *mut f32, self.num_elements as usize) }
    }

    #[inline]
    fn cfvalues(&self) -> &[f32] {
        unsafe { slice::from_raw_parts(self.storage2 as *const f32, self.num_elements as usize) }
    }

    #[inline]
    fn cfvalues_mut(&mut self) -> &mut [f32] {
        unsafe { slice::from_raw_parts_mut(self.storage2 as *mut f32, self.num_elements as usize) }
    }

    #[inline]
    fn has_cfvalues_ip(&self) -> bool {
        self.num_elements_ip != 0
    }

    #[inline]
    fn cfvalues_ip(&self) -> &[f32] {
        unsafe { slice::from_raw_parts(self.storage3 as *const f32, self.num_elements_ip as usize) }
    }

    #[inline]
    fn cfvalues_ip_mut(&mut self) -> &mut [f32] {
        unsafe {
            slice::from_raw_parts_mut(self.storage3 as *mut f32, self.num_elements_ip as usize)
        }
    }

    #[inline]
    fn cfvalues_chance(&self) -> &[f32] {
        unsafe { slice::from_raw_parts(self.storage1 as *const f32, self.num_elements as usize) }
    }

    #[inline]
    fn cfvalues_chance_mut(&mut self) -> &mut [f32] {
        unsafe { slice::from_raw_parts_mut(self.storage1 as *mut f32, self.num_elements as usize) }
    }

    #[inline]
    fn strategy_compressed(&self) -> &[u16] {
        unsafe { slice::from_raw_parts(self.storage1 as *const u16, self.num_elements as usize) }
    }

    #[inline]
    fn strategy_compressed_mut(&mut self) -> &mut [u16] {
        unsafe { slice::from_raw_parts_mut(self.storage1 as *mut u16, self.num_elements as usize) }
    }

    #[inline]
    fn regrets_compressed(&self) -> &[i16] {
        unsafe { slice::from_raw_parts(self.storage2 as *const i16, self.num_elements as usize) }
    }

    #[inline]
    fn regrets_compressed_mut(&mut self) -> &mut [i16] {
        unsafe { slice::from_raw_parts_mut(self.storage2 as *mut i16, self.num_elements as usize) }
    }

    #[inline]
    fn cfvalues_compressed(&self) -> &[i16] {
        unsafe { slice::from_raw_parts(self.storage2 as *const i16, self.num_elements as usize) }
    }

    #[inline]
    fn cfvalues_compressed_mut(&mut self) -> &mut [i16] {
        unsafe { slice::from_raw_parts_mut(self.storage2 as *mut i16, self.num_elements as usize) }
    }

    #[inline]
    fn cfvalues_ip_compressed(&self) -> &[i16] {
        unsafe { slice::from_raw_parts(self.storage3 as *const i16, self.num_elements_ip as usize) }
    }

    #[inline]
    fn cfvalues_ip_compressed_mut(&mut self) -> &mut [i16] {
        unsafe {
            slice::from_raw_parts_mut(self.storage3 as *mut i16, self.num_elements_ip as usize)
        }
    }

    #[inline]
    fn cfvalues_chance_compressed(&self) -> &[i16] {
        unsafe { slice::from_raw_parts(self.storage1 as *const i16, self.num_elements as usize) }
    }

    #[inline]
    fn cfvalues_chance_compressed_mut(&mut self) -> &mut [i16] {
        unsafe { slice::from_raw_parts_mut(self.storage1 as *mut i16, self.num_elements as usize) }
    }

    #[inline]
    fn strategy_scale(&self) -> f32 {
        self.scale1
    }

    #[inline]
    fn set_strategy_scale(&mut self, scale: f32) {
        self.scale1 = scale;
    }

    #[inline]
    fn regret_scale(&self) -> f32 {
        self.scale2
    }

    #[inline]
    fn set_regret_scale(&mut self, scale: f32) {
        self.scale2 = scale;
    }

    #[inline]
    fn cfvalue_scale(&self) -> f32 {
        self.scale2
    }

    #[inline]
    fn set_cfvalue_scale(&mut self, scale: f32) {
        self.scale2 = scale;
    }

    #[inline]
    fn cfvalue_ip_scale(&self) -> f32 {
        self.scale3
    }

    #[inline]
    fn set_cfvalue_ip_scale(&mut self, scale: f32) {
        self.scale3 = scale;
    }

    #[inline]
    fn cfvalue_chance_scale(&self) -> f32 {
        self.scale1
    }

    #[inline]
    fn set_cfvalue_chance_scale(&mut self, scale: f32) {
        self.scale1 = scale;
    }

    #[inline]
    fn enable_parallelization(&self) -> bool {
        self.river == NOT_DEALT
    }

    fn my_end_range(&self, game: &PostFlopGame) -> Vec<f32>
    {
        const VERBOSE: bool = false;
        const RANGE_LEN: usize = 52 * 51 / 2;

        if self.mstorage_offset == u32::MAX
        {
            return vec![] as Vec<f32>
        }

        let mr_storage = unsafe { game.mrstorage.yoink() };

        if VERBOSE { println!("my_end_range: Initializing pointer getting sequence"); }
        if VERBOSE { println!("my_end_range: offset for mr_storage: {}", self.mstorage_offset); }

        let m_ptr = unsafe { mr_storage.as_ptr().add(self.mstorage_offset as usize) };

        if VERBOSE { println!("my_end_range: target acquired"); }

        let m_slice: &[u32] = unsafe {slice::from_raw_parts(m_ptr, self.num_actions())};
        let mut r_vec: Vec<f32> = vec![];

        if VERBOSE { println!("my_end_range: slice received"); }
        
        if VERBOSE { println!("my_end_range: slice-contained data: {:?}", m_slice); }

        for i in 0..self.num_actions()
        {
            if VERBOSE { println!("my_end_range: initializing offset capture protocol"); }

            let raw_offset = m_slice[i];
            let normal_ptr = unsafe { game.rstorage.yoink().as_ptr().offset(raw_offset as isize) } as *mut f32;

            if VERBOSE { println!("my_end_range: float coordinates discovered, proceeding with slice capture"); }

            assert!(!normal_ptr.is_null(), "normal pointer is null! Yuck!");

            let slice: &[f32] = unsafe {slice::from_raw_parts(normal_ptr, RANGE_LEN)};

            if VERBOSE { println!("my_end_range: slice successfully captured, transferring it into containment vector"); }

            r_vec.extend_from_slice(slice);

            if VERBOSE { println!("my_end_range: transfer finished"); }
        }

        if VERBOSE { println!("my_end_range: mission accomplished"); }

        r_vec
    }

    fn my_end_limit(&self, game: &PostFlopGame) -> Vec<i8>
    {
        const VERBOSE: bool = false;
        const RANGE_LEN: usize = 52 * 51 / 2;

        if self.mstorage_offset == u32::MAX
        {
            return vec![] as Vec<i8>
        }

        let ml_storage = unsafe { game.mlstorage.yoink() };

        if VERBOSE { println!("my_end_limit: Initializing pointer getting sequence"); }
        if VERBOSE { println!("my_end_limit: offset for mr_storage: {}", self.mstorage_offset); }

        let m_ptr = unsafe { ml_storage.as_ptr().add(self.mstorage_offset as usize) };

        if VERBOSE { println!("my_end_limit: target acquired"); }

        let m_slice: &[u32] = unsafe {slice::from_raw_parts(m_ptr, self.num_actions())};
        let mut l_vec: Vec<i8> = vec![];

        if VERBOSE { println!("my_end_limit: slice received"); }
        
        if VERBOSE { println!("my_end_limit: slice-contained data: {:?}", m_slice); }

        for i in 0..self.num_actions()
        {
            if VERBOSE { println!("my_end_limit: initializing offset capture protocol"); }

            let raw_offset = m_slice[i];
            let normal_ptr = unsafe { game.lstorage.yoink().as_ptr().offset(raw_offset as isize) } as *mut i8;

            if VERBOSE { println!("my_end_limit: float coordinates discovered, proceeding with slice capture"); }

            assert!(!normal_ptr.is_null(), "normal pointer is null! Yuck!");

            let slice: &[i8] = unsafe {slice::from_raw_parts(normal_ptr, RANGE_LEN)};

            if VERBOSE { println!("my_end_limit: slice successfully captured, transferring it into containment vector"); }

            l_vec.extend_from_slice(slice);

            if VERBOSE { println!("my_end_limit: transfer finished"); }
        }

        if VERBOSE { println!("my_end_limit: mission accomplished"); }

        l_vec
    }

    fn my_boni(&self) -> Vec<u8> {
        let mut boni = vec![] as Vec<u8>;
        if self.turn != NOT_DEALT
        {
            boni.push(self.turn);
        }
        if self.river != NOT_DEALT
        {
            boni.push(self.river);
        }
        boni
    }
}

impl Default for PostFlopNode {
    #[inline]
    fn default() -> Self {
        Self {
            prev_action: Action::None,
            player: PLAYER_OOP,
            turn: NOT_DEALT,
            river: NOT_DEALT,
            is_locked: false,
            mstorage_offset: u32::MAX,
            amount: 0,
            children_offset: 0,
            num_children: 0,
            num_elements_ip: 0,
            storage1: ptr::null_mut(),
            storage2: ptr::null_mut(),
            storage3: ptr::null_mut(),
            num_elements: 0,
            scale1: 0.0,
            scale2: 0.0,
            scale3: 0.0,
        }
    }
}

impl PostFlopNode {
    #[inline]
    pub(super) fn children(&self) -> &[MutexLike<Self>] {
        // This is safe because `MutexLike<T>` is a `repr(transparent)` wrapper around `T`.
        let self_ptr = self as *const _ as *const MutexLike<PostFlopNode>;
        unsafe {
            slice::from_raw_parts(
                self_ptr.add(self.children_offset as usize),
                self.num_children as usize,
            )
        }
    }
}


impl GamePair for PostFlopPair {
    type G = PostFlopGame;
    type N = PostFlopNode;
}
