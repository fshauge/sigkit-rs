// generated from https://github.com/Vector35/sigkit/blob/e5f625c23b96bc1fa1b939d86bf50712fa22af85/signaturelibrary.fbs

#![allow(dead_code)]

use flatbuffers::EndianScalar;

// struct CallRef, aligned to 4
#[derive(Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct CallRef(pub [u8; 8]);

impl Default for CallRef {
    fn default() -> Self {
        Self([0; 8])
    }
}

impl core::fmt::Debug for CallRef {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CallRef")
            .field("offset", &self.offset())
            .field("dst_id", &self.dst_id())
            .finish()
    }
}

impl flatbuffers::SimpleToVerifyInSlice for CallRef {}

impl<'a> flatbuffers::Follow<'a> for CallRef {
    type Inner = &'a CallRef;

    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        <&'a CallRef>::follow(buf, loc)
    }
}

impl<'a> flatbuffers::Follow<'a> for &'a CallRef {
    type Inner = &'a CallRef;

    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        flatbuffers::follow_cast_ref::<CallRef>(buf, loc)
    }
}

impl<'b> flatbuffers::Push for CallRef {
    type Output = CallRef;

    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        let src = ::core::slice::from_raw_parts(self as *const CallRef as *const u8, Self::size());
        dst.copy_from_slice(src);
    }
}

impl<'a> flatbuffers::Verifiable for CallRef {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        v.in_buffer::<Self>(pos)
    }
}

impl<'a> CallRef {
    #[allow(clippy::too_many_arguments)]
    pub fn new(offset: i32, dst_id: i32) -> Self {
        let mut s = Self([0; 8]);
        s.set_offset(offset);
        s.set_dst_id(dst_id);
        s
    }

    pub fn offset(&self) -> i32 {
        let mut mem = core::mem::MaybeUninit::<<i32 as EndianScalar>::Scalar>::uninit();
        // Safety:
        // Created from a valid Table for this object
        // Which contains a valid value in this slot
        EndianScalar::from_little_endian(unsafe {
            core::ptr::copy_nonoverlapping(
                self.0[0..].as_ptr(),
                mem.as_mut_ptr() as *mut u8,
                core::mem::size_of::<<i32 as EndianScalar>::Scalar>(),
            );
            mem.assume_init()
        })
    }

    pub fn set_offset(&mut self, x: i32) {
        let x_le = x.to_little_endian();
        // Safety:
        // Created from a valid Table for this object
        // Which contains a valid value in this slot
        unsafe {
            core::ptr::copy_nonoverlapping(
                &x_le as *const _ as *const u8,
                self.0[0..].as_mut_ptr(),
                core::mem::size_of::<<i32 as EndianScalar>::Scalar>(),
            );
        }
    }

    #[inline]
    pub fn key_compare_less_than(&self, o: &CallRef) -> bool {
        self.offset() < o.offset()
    }

    #[inline]
    pub fn key_compare_with_value(&self, val: i32) -> ::core::cmp::Ordering {
        let key = self.offset();
        key.cmp(&val)
    }

    pub fn dst_id(&self) -> i32 {
        let mut mem = core::mem::MaybeUninit::<<i32 as EndianScalar>::Scalar>::uninit();
        // Safety:
        // Created from a valid Table for this object
        // Which contains a valid value in this slot
        EndianScalar::from_little_endian(unsafe {
            core::ptr::copy_nonoverlapping(
                self.0[4..].as_ptr(),
                mem.as_mut_ptr() as *mut u8,
                core::mem::size_of::<<i32 as EndianScalar>::Scalar>(),
            );
            mem.assume_init()
        })
    }

    pub fn set_dst_id(&mut self, x: i32) {
        let x_le = x.to_little_endian();
        // Safety:
        // Created from a valid Table for this object
        // Which contains a valid value in this slot
        unsafe {
            core::ptr::copy_nonoverlapping(
                &x_le as *const _ as *const u8,
                self.0[4..].as_mut_ptr(),
                core::mem::size_of::<<i32 as EndianScalar>::Scalar>(),
            );
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum PatternOffset {}

pub struct Pattern<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Pattern<'a> {
    type Inner = Pattern<'a>;

    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table::new(buf, loc),
        }
    }
}

impl<'a> Pattern<'a> {
    pub const VT_DATA: flatbuffers::VOffsetT = 4;
    pub const VT_MASK: flatbuffers::VOffsetT = 6;

    #[inline]
    pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Pattern { _tab: table }
    }

    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args PatternArgs<'args>,
    ) -> flatbuffers::WIPOffset<Pattern<'bldr>> {
        let mut builder = PatternBuilder::new(_fbb);
        if let Some(x) = args.mask {
            builder.add_mask(x);
        }
        if let Some(x) = args.data {
            builder.add_data(x);
        }
        builder.finish()
    }

    #[inline]
    pub fn data(&self) -> flatbuffers::Vector<'a, u8> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(
                    Pattern::VT_DATA,
                    None,
                )
                .unwrap()
        }
    }

    #[inline]
    pub fn mask(&self) -> flatbuffers::Vector<'a, u8> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(
                    Pattern::VT_MASK,
                    None,
                )
                .unwrap()
        }
    }
}

impl flatbuffers::Verifiable for Pattern<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>(
                "data",
                Self::VT_DATA,
                true,
            )?
            .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>(
                "mask",
                Self::VT_MASK,
                true,
            )?
            .finish();
        Ok(())
    }
}

pub struct PatternArgs<'a> {
    pub data: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
    pub mask: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
}

impl<'a> Default for PatternArgs<'a> {
    #[inline]
    fn default() -> Self {
        PatternArgs {
            data: None, // required field
            mask: None, // required field
        }
    }
}

pub struct PatternBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}

impl<'a: 'b, 'b> PatternBuilder<'a, 'b> {
    #[inline]
    pub fn add_data(&mut self, data: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(Pattern::VT_DATA, data);
    }

    #[inline]
    pub fn add_mask(&mut self, mask: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(Pattern::VT_MASK, mask);
    }

    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> PatternBuilder<'a, 'b> {
        let start = _fbb.start_table();
        PatternBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }

    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<Pattern<'a>> {
        let o = self.fbb_.end_table(self.start_);
        self.fbb_.required(o, Pattern::VT_DATA, "data");
        self.fbb_.required(o, Pattern::VT_MASK, "mask");
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl core::fmt::Debug for Pattern<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut ds = f.debug_struct("Pattern");
        ds.field("data", &self.data());
        ds.field("mask", &self.mask());
        ds.finish()
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum FunctionOffset {}

pub struct Function<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Function<'a> {
    type Inner = Function<'a>;

    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table::new(buf, loc),
        }
    }
}

impl<'a> Function<'a> {
    pub const VT_NAME: flatbuffers::VOffsetT = 4;
    pub const VT_SOURCE_BINARY: flatbuffers::VOffsetT = 6;
    pub const VT_CALLEES: flatbuffers::VOffsetT = 8;
    pub const VT_PATTERN: flatbuffers::VOffsetT = 10;
    pub const VT_PATTERN_OFFSET: flatbuffers::VOffsetT = 12;
    pub const VT_IS_BRIDGE: flatbuffers::VOffsetT = 14;

    #[inline]
    pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Function { _tab: table }
    }

    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args FunctionArgs<'args>,
    ) -> flatbuffers::WIPOffset<Function<'bldr>> {
        let mut builder = FunctionBuilder::new(_fbb);
        builder.add_pattern_offset(args.pattern_offset);
        if let Some(x) = args.pattern {
            builder.add_pattern(x);
        }
        if let Some(x) = args.callees {
            builder.add_callees(x);
        }
        if let Some(x) = args.source_binary {
            builder.add_source_binary(x);
        }
        if let Some(x) = args.name {
            builder.add_name(x);
        }
        builder.add_is_bridge(args.is_bridge);
        builder.finish()
    }

    #[inline]
    pub fn name(&self) -> &'a str {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<&str>>(Function::VT_NAME, None)
                .unwrap()
        }
    }

    #[inline]
    pub fn source_binary(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<&str>>(Function::VT_SOURCE_BINARY, None)
        }
    }

    #[inline]
    pub fn callees(&self) -> Option<flatbuffers::Vector<'a, CallRef>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, CallRef>>>(
                    Function::VT_CALLEES,
                    None,
                )
        }
    }

    #[inline]
    pub fn pattern(&self) -> Option<Pattern<'a>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<Pattern>>(Function::VT_PATTERN, None)
        }
    }

    #[inline]
    pub fn pattern_offset(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(Function::VT_PATTERN_OFFSET, Some(0))
                .unwrap()
        }
    }

    #[inline]
    pub fn is_bridge(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(Function::VT_IS_BRIDGE, Some(false))
                .unwrap()
        }
    }
}

impl flatbuffers::Verifiable for Function<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<flatbuffers::ForwardsUOffset<&str>>("name", Self::VT_NAME, true)?
            .visit_field::<flatbuffers::ForwardsUOffset<&str>>(
                "source_binary",
                Self::VT_SOURCE_BINARY,
                false,
            )?
            .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, CallRef>>>(
                "callees",
                Self::VT_CALLEES,
                false,
            )?
            .visit_field::<flatbuffers::ForwardsUOffset<Pattern>>(
                "pattern",
                Self::VT_PATTERN,
                false,
            )?
            .visit_field::<u32>("pattern_offset", Self::VT_PATTERN_OFFSET, false)?
            .visit_field::<bool>("is_bridge", Self::VT_IS_BRIDGE, false)?
            .finish();
        Ok(())
    }
}

pub struct FunctionArgs<'a> {
    pub name: Option<flatbuffers::WIPOffset<&'a str>>,
    pub source_binary: Option<flatbuffers::WIPOffset<&'a str>>,
    pub callees: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, CallRef>>>,
    pub pattern: Option<flatbuffers::WIPOffset<Pattern<'a>>>,
    pub pattern_offset: u32,
    pub is_bridge: bool,
}

impl<'a> Default for FunctionArgs<'a> {
    #[inline]
    fn default() -> Self {
        FunctionArgs {
            name: None, // required field
            source_binary: None,
            callees: None,
            pattern: None,
            pattern_offset: 0,
            is_bridge: false,
        }
    }
}

pub struct FunctionBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}

impl<'a: 'b, 'b> FunctionBuilder<'a, 'b> {
    #[inline]
    pub fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b str>) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(Function::VT_NAME, name);
    }

    #[inline]
    pub fn add_source_binary(&mut self, source_binary: flatbuffers::WIPOffset<&'b str>) {
        self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
            Function::VT_SOURCE_BINARY,
            source_binary,
        );
    }

    #[inline]
    pub fn add_callees(
        &mut self,
        callees: flatbuffers::WIPOffset<flatbuffers::Vector<'b, CallRef>>,
    ) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(Function::VT_CALLEES, callees);
    }

    #[inline]
    pub fn add_pattern(&mut self, pattern: flatbuffers::WIPOffset<Pattern<'b>>) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<Pattern>>(Function::VT_PATTERN, pattern);
    }

    #[inline]
    pub fn add_pattern_offset(&mut self, pattern_offset: u32) {
        self.fbb_
            .push_slot::<u32>(Function::VT_PATTERN_OFFSET, pattern_offset, 0);
    }

    #[inline]
    pub fn add_is_bridge(&mut self, is_bridge: bool) {
        self.fbb_
            .push_slot::<bool>(Function::VT_IS_BRIDGE, is_bridge, false);
    }

    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> FunctionBuilder<'a, 'b> {
        let start = _fbb.start_table();
        FunctionBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }

    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<Function<'a>> {
        let o = self.fbb_.end_table(self.start_);
        self.fbb_.required(o, Function::VT_NAME, "name");
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl core::fmt::Debug for Function<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut ds = f.debug_struct("Function");
        ds.field("name", &self.name());
        ds.field("source_binary", &self.source_binary());
        ds.field("callees", &self.callees());
        ds.field("pattern", &self.pattern());
        ds.field("pattern_offset", &self.pattern_offset());
        ds.field("is_bridge", &self.is_bridge());
        ds.finish()
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum TrieNodeOffset {}

pub struct TrieNode<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for TrieNode<'a> {
    type Inner = TrieNode<'a>;

    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table::new(buf, loc),
        }
    }
}

impl<'a> TrieNode<'a> {
    pub const VT_PATTERN_PREFIX: flatbuffers::VOffsetT = 4;
    pub const VT_PATTERN: flatbuffers::VOffsetT = 6;
    pub const VT_CHILDREN: flatbuffers::VOffsetT = 8;
    pub const VT_WILDCARD_CHILD: flatbuffers::VOffsetT = 10;
    pub const VT_FUNCTIONS: flatbuffers::VOffsetT = 12;

    #[inline]
    pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        TrieNode { _tab: table }
    }

    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args TrieNodeArgs<'args>,
    ) -> flatbuffers::WIPOffset<TrieNode<'bldr>> {
        let mut builder = TrieNodeBuilder::new(_fbb);
        if let Some(x) = args.functions {
            builder.add_functions(x);
        }
        if let Some(x) = args.wildcard_child {
            builder.add_wildcard_child(x);
        }
        if let Some(x) = args.children {
            builder.add_children(x);
        }
        if let Some(x) = args.pattern {
            builder.add_pattern(x);
        }
        builder.add_pattern_prefix(args.pattern_prefix);
        builder.finish()
    }

    #[inline]
    pub fn pattern_prefix(&self) -> u8 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u8>(TrieNode::VT_PATTERN_PREFIX, Some(0))
                .unwrap()
        }
    }

    #[inline]
    pub fn key_compare_less_than(&self, o: &TrieNode) -> bool {
        self.pattern_prefix() < o.pattern_prefix()
    }

    #[inline]
    pub fn key_compare_with_value(&self, val: u8) -> ::core::cmp::Ordering {
        let key = self.pattern_prefix();
        key.cmp(&val)
    }

    #[inline]
    pub fn pattern(&self) -> Pattern<'a> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<Pattern>>(TrieNode::VT_PATTERN, None)
                .unwrap()
        }
    }

    #[inline]
    pub fn children(
        &self,
    ) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<TrieNode<'a>>>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<TrieNode>>,
            >>(TrieNode::VT_CHILDREN, None)
        }
    }

    #[inline]
    pub fn wildcard_child(&self) -> Option<TrieNode<'a>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<TrieNode>>(TrieNode::VT_WILDCARD_CHILD, None)
        }
    }

    #[inline]
    pub fn functions(&self) -> Option<flatbuffers::Vector<'a, u32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u32>>>(
                    TrieNode::VT_FUNCTIONS,
                    None,
                )
        }
    }
}

impl flatbuffers::Verifiable for TrieNode<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<u8>("pattern_prefix", Self::VT_PATTERN_PREFIX, false)?
            .visit_field::<flatbuffers::ForwardsUOffset<Pattern>>(
                "pattern",
                Self::VT_PATTERN,
                true,
            )?
            .visit_field::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<TrieNode>>,
            >>("children", Self::VT_CHILDREN, false)?
            .visit_field::<flatbuffers::ForwardsUOffset<TrieNode>>(
                "wildcard_child",
                Self::VT_WILDCARD_CHILD,
                false,
            )?
            .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u32>>>(
                "functions",
                Self::VT_FUNCTIONS,
                false,
            )?
            .finish();
        Ok(())
    }
}

pub struct TrieNodeArgs<'a> {
    pub pattern_prefix: u8,
    pub pattern: Option<flatbuffers::WIPOffset<Pattern<'a>>>,
    pub children: Option<
        flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<TrieNode<'a>>>>,
    >,
    pub wildcard_child: Option<flatbuffers::WIPOffset<TrieNode<'a>>>,
    pub functions: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u32>>>,
}

impl<'a> Default for TrieNodeArgs<'a> {
    #[inline]
    fn default() -> Self {
        TrieNodeArgs {
            pattern_prefix: 0,
            pattern: None, // required field
            children: None,
            wildcard_child: None,
            functions: None,
        }
    }
}

pub struct TrieNodeBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}

impl<'a: 'b, 'b> TrieNodeBuilder<'a, 'b> {
    #[inline]
    pub fn add_pattern_prefix(&mut self, pattern_prefix: u8) {
        self.fbb_
            .push_slot::<u8>(TrieNode::VT_PATTERN_PREFIX, pattern_prefix, 0);
    }

    #[inline]
    pub fn add_pattern(&mut self, pattern: flatbuffers::WIPOffset<Pattern<'b>>) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<Pattern>>(TrieNode::VT_PATTERN, pattern);
    }

    #[inline]
    pub fn add_children(
        &mut self,
        children: flatbuffers::WIPOffset<
            flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<TrieNode<'b>>>,
        >,
    ) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(TrieNode::VT_CHILDREN, children);
    }

    #[inline]
    pub fn add_wildcard_child(&mut self, wildcard_child: flatbuffers::WIPOffset<TrieNode<'b>>) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<TrieNode>>(
                TrieNode::VT_WILDCARD_CHILD,
                wildcard_child,
            );
    }

    #[inline]
    pub fn add_functions(
        &mut self,
        functions: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u32>>,
    ) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(TrieNode::VT_FUNCTIONS, functions);
    }

    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TrieNodeBuilder<'a, 'b> {
        let start = _fbb.start_table();
        TrieNodeBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }

    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<TrieNode<'a>> {
        let o = self.fbb_.end_table(self.start_);
        self.fbb_.required(o, TrieNode::VT_PATTERN, "pattern");
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl core::fmt::Debug for TrieNode<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut ds = f.debug_struct("TrieNode");
        ds.field("pattern_prefix", &self.pattern_prefix());
        ds.field("pattern", &self.pattern());
        ds.field("children", &self.children());
        ds.field("wildcard_child", &self.wildcard_child());
        ds.field("functions", &self.functions());
        ds.finish()
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum SignatureLibraryOffset {}

pub struct SignatureLibrary<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for SignatureLibrary<'a> {
    type Inner = SignatureLibrary<'a>;

    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table::new(buf, loc),
        }
    }
}

impl<'a> SignatureLibrary<'a> {
    pub const VT_FUNCTIONS: flatbuffers::VOffsetT = 4;
    pub const VT_ROOT: flatbuffers::VOffsetT = 6;

    #[inline]
    pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        SignatureLibrary { _tab: table }
    }

    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args SignatureLibraryArgs<'args>,
    ) -> flatbuffers::WIPOffset<SignatureLibrary<'bldr>> {
        let mut builder = SignatureLibraryBuilder::new(_fbb);
        if let Some(x) = args.root {
            builder.add_root(x);
        }
        if let Some(x) = args.functions {
            builder.add_functions(x);
        }
        builder.finish()
    }

    #[inline]
    pub fn functions(&self) -> flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Function<'a>>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<
                    flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Function>>,
                >>(SignatureLibrary::VT_FUNCTIONS, None)
                .unwrap()
        }
    }

    #[inline]
    pub fn root(&self) -> TrieNode<'a> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<TrieNode>>(SignatureLibrary::VT_ROOT, None)
                .unwrap()
        }
    }
}

impl flatbuffers::Verifiable for SignatureLibrary<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<Function>>,
            >>("functions", Self::VT_FUNCTIONS, true)?
            .visit_field::<flatbuffers::ForwardsUOffset<TrieNode>>("root", Self::VT_ROOT, true)?
            .finish();
        Ok(())
    }
}

pub struct SignatureLibraryArgs<'a> {
    pub functions: Option<
        flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Function<'a>>>>,
    >,
    pub root: Option<flatbuffers::WIPOffset<TrieNode<'a>>>,
}

impl<'a> Default for SignatureLibraryArgs<'a> {
    #[inline]
    fn default() -> Self {
        SignatureLibraryArgs {
            functions: None, // required field
            root: None,      // required field
        }
    }
}

pub struct SignatureLibraryBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}

impl<'a: 'b, 'b> SignatureLibraryBuilder<'a, 'b> {
    #[inline]
    pub fn add_functions(
        &mut self,
        functions: flatbuffers::WIPOffset<
            flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<Function<'b>>>,
        >,
    ) {
        self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
            SignatureLibrary::VT_FUNCTIONS,
            functions,
        );
    }

    #[inline]
    pub fn add_root(&mut self, root: flatbuffers::WIPOffset<TrieNode<'b>>) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<TrieNode>>(SignatureLibrary::VT_ROOT, root);
    }

    #[inline]
    pub fn new(
        _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> SignatureLibraryBuilder<'a, 'b> {
        let start = _fbb.start_table();
        SignatureLibraryBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }

    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<SignatureLibrary<'a>> {
        let o = self.fbb_.end_table(self.start_);
        self.fbb_
            .required(o, SignatureLibrary::VT_FUNCTIONS, "functions");
        self.fbb_.required(o, SignatureLibrary::VT_ROOT, "root");
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl core::fmt::Debug for SignatureLibrary<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut ds = f.debug_struct("SignatureLibrary");
        ds.field("functions", &self.functions());
        ds.field("root", &self.root());
        ds.finish()
    }
}

/// Verifies that a buffer of bytes contains a `SignatureLibrary`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_signature_library_unchecked`.
#[inline]
pub fn root_as_signature_library(
    buf: &[u8],
) -> Result<SignatureLibrary, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::root::<SignatureLibrary>(buf)
}

/// Verifies that a buffer of bytes contains a size prefixed
/// `SignatureLibrary` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_signature_library_unchecked`.
#[inline]
pub fn size_prefixed_root_as_signature_library(
    buf: &[u8],
) -> Result<SignatureLibrary, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::size_prefixed_root::<SignatureLibrary>(buf)
}

/// Verifies, with the given options, that a buffer of bytes
/// contains a `SignatureLibrary` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_signature_library_unchecked`.
#[inline]
pub fn root_as_signature_library_with_opts<'b, 'o>(
    opts: &'o flatbuffers::VerifierOptions,
    buf: &'b [u8],
) -> Result<SignatureLibrary<'b>, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::root_with_opts::<SignatureLibrary<'b>>(opts, buf)
}

/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `SignatureLibrary` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_signature_library_unchecked`.
#[inline]
pub fn size_prefixed_root_as_signature_library_with_opts<'b, 'o>(
    opts: &'o flatbuffers::VerifierOptions,
    buf: &'b [u8],
) -> Result<SignatureLibrary<'b>, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::size_prefixed_root_with_opts::<SignatureLibrary<'b>>(opts, buf)
}

/// Assumes, without verification, that a buffer of bytes contains a SignatureLibrary and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `SignatureLibrary`.
#[inline]
pub unsafe fn root_as_signature_library_unchecked(buf: &[u8]) -> SignatureLibrary {
    flatbuffers::root_unchecked::<SignatureLibrary>(buf)
}

/// Assumes, without verification, that a buffer of bytes contains a size prefixed SignatureLibrary and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `SignatureLibrary`.
#[inline]
pub unsafe fn size_prefixed_root_as_signature_library_unchecked(buf: &[u8]) -> SignatureLibrary {
    flatbuffers::size_prefixed_root_unchecked::<SignatureLibrary>(buf)
}

pub const SIGNATURE_LIBRARY_EXTENSION: &str = "sig";

#[inline]
pub fn finish_signature_library_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<SignatureLibrary<'a>>,
) {
    fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_signature_library_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<SignatureLibrary<'a>>,
) {
    fbb.finish_size_prefixed(root, None);
}
