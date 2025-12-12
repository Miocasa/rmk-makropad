use rmk::types::action::{EncoderAction, KeyAction};
use rmk::{a, encoder, k, layer, mo};
pub(crate) const COL: usize = 3;
pub(crate) const ROW: usize = 4;
pub(crate) const NUM_LAYER: usize = 8;
pub(crate) const NUM_ENCODER: usize = 2;

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        layer!([
            [k!(A), k!(B), k!(C)],
            [k!(Kc1), k!(Kc2), k!(Kc3)],
            [k!(LCtrl), mo!(1), k!(LShift)],
            [osl!(1), lt!(2, Kc9), lm!(1, LShift | LGui)]
        ]),
        layer!([
            [k!(A), k!(B), k!(C)],
            [k!(Kc1), k!(Kc2), k!(Kc3)],
            [k!(LCtrl), mo!(1), k!(LShift)],
            [osl!(1), lt!(2, Kc9), lm!(1, LShift | LGui)]
        ]),
        layer!([
            [k!(A), k!(B), k!(C)],
            [k!(Kc1), k!(Kc2), k!(Kc3)],
            [k!(LCtrl), mo!(1), k!(LShift)],
            [osl!(1), lt!(2, Kc9), lm!(1, LShift | LGui)]
        ]),
        layer!([
            [k!(A), k!(B), k!(C)],
            [k!(Kc1), k!(Kc2), k!(Kc3)],
            [k!(LCtrl), mo!(1), k!(LShift)],
            [osl!(1), lt!(2, Kc9), lm!(1, LShift | LGui)]
        ]),
        layer!([
            [k!(A), k!(B), k!(C)],
            [k!(Kc1), k!(Kc2), k!(Kc3)],
            [k!(LCtrl), mo!(1), k!(LShift)],
            [osl!(1), lt!(2, Kc9), lm!(1, LShift | LGui)]
        ]),
    ]
}

pub const fn get_default_encoder_map() -> [[EncoderAction; NUM_ENCODER]; NUM_LAYER] {
    [
        [
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
        ],
        [
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
        ],
        [
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
        ],
        [
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
        ],
        [
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
        ],
        [
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
        ],
        [
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
        ],
        [
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
        ],
    ]
}
