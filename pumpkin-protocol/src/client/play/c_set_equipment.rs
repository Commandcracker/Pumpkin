use crate::bytebuf::ByteBuffer;
use crate::slot::Slot;
use crate::{ClientPacket, VarInt};
use pumpkin_macros::client_packet;
use serde::Serialize;

#[client_packet("play:set_equipment")]
pub struct CSetEquipment {
    entity_id: VarInt,
    equipment: Vec<Equipment>,
}

impl CSetEquipment {
    pub fn new(entity_id: VarInt, equipment: Vec<Equipment>) -> Self {
        Self {
            entity_id,
            equipment,
        }
    }
}

impl ClientPacket for CSetEquipment {
    fn write(&self, bytebuf: &mut ByteBuffer) {
        bytebuf.put_var_int(&self.entity_id);
        let len = self.equipment.len();

        for (index, equip) in self.equipment.iter().enumerate() {
            let is_not_last = index != len - 1;
            let slot_value: i8 = equip.slot.clone() as i8;
            bytebuf.put_i8(if is_not_last {
                slot_value | -128 // Set the highest bit if not the last item
            } else {
                slot_value
            });
            // put equip.item (Slot)
        }
    }
}


#[derive(Serialize)]
pub struct Equipment {
    pub slot: EquipmentSlot,
    pub item: Slot,
}

impl Equipment {
    pub fn new(slot: EquipmentSlot, item: Slot) -> Self {
        Self { slot, item }
    }
}

#[derive(Serialize, Clone)]
#[repr(i8)]
pub enum EquipmentSlot {
    MainHand = 0,
    OffHand = 1,
    Boots = 2,
    Leggings = 3,
    ChestPlate = 4,
    Helmet = 5,
    Body = 6,
}
