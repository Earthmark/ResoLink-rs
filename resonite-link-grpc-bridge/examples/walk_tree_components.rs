use crate::pb::*;

mod pb {
    tonic::include_proto!("rfmk.resonite");
}

/// An example useful for field debugging where every slot is queried for all components
/// after a root level query gets all slots.
/// This will walk all fields, but only in the context of a single slot.
/// This helps narrow down what components may be having parser errors.

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client =
        &mut resonite_link_client::ResoniteLinkClient::connect("grpc://127.0.0.1:8080").await?;

    let tree = client
        .get_slot(GetSlotRequest {
            slot_id: "Root".into(),
            include_component_data: false,
            depth: 10,
        })
        .await?
        .into_inner();

    let mut slots = Vec::new();

    gather_children(&tree.data.as_ref().unwrap(), &mut slots);

    for slot in slots {
        println!(
            "Reading components for {} (ID: {})",
            slot.name.as_ref().unwrap().value.as_ref().unwrap(),
            slot.id,
        );

        client
            .get_slot(GetSlotRequest {
                slot_id: slot.id.clone(),
                include_component_data: true,
                depth: 0,
            })
            .await?;
    }

    Ok(())
}

fn gather_children<'a>(slot: &'a Slot, slots: &mut Vec<&'a Slot>) {
    slots.push(&slot);
    for child in &slot.children {
        gather_children(child, slots);
    }
}
