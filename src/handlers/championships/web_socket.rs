use crate::{
    dtos::F123Data,
    entity::Championship,
    error::{AppResult, ChampionshipError},
    states::UserState,
};
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    response::Response, // IntoResponse
};
use std::time::Duration;
use tokio::time::sleep;

#[inline(always)]
pub async fn session_socket(
    State(state): State<UserState>,
    Path((championship_id, session_id)): Path<(i32, i64)>,
    ws: WebSocketUpgrade,
) -> AppResult<Response> {
    let championship = state.championship_repository.find(&championship_id).await?;
    let session_exists = state
        .championship_repository
        .session_exists(&championship_id, session_id)
        .await?;

    if !session_exists {
        return Err(ChampionshipError::SessionNotFound)?;
    }

    Ok(ws.on_upgrade(move |socket| handle_socket(socket, state, championship, session_id)))
}

#[inline(always)]
async fn handle_socket(
    mut socket: WebSocket,
    state: UserState,
    championship: Championship,
    _session_id: i64,
) {
    // TODO: Implement all the socket logic (Only Send data)
    loop {
        let Some((packet_id, data)) = state.f123_service.get_receiver(&championship.id).await
        else {
            sleep(Duration::from_millis(700)).await;
            continue;
        };

        let Ok(Some(packet)) = F123Data::deserialize(packet_id.into(), data.as_slice()) else {
            sleep(Duration::from_millis(700)).await;
            continue;
        };

        match packet {
            F123Data::Motion(motion_data) => {
                let data = rmp_serde::to_vec(&motion_data.m_carMotionData).unwrap();
                let _ = socket.send(Message::Binary(data)).await;
            }

            F123Data::SessionHistory(history_data) => {
                let data = rmp_serde::to_vec(&history_data).unwrap();
                let _ = socket.send(Message::Binary(data)).await;
            }

            F123Data::Session(session_data) => {
                let data = rmp_serde::to_vec(&session_data).unwrap();
                let _ = socket.send(Message::Binary(data)).await;
            }

            F123Data::Event(event) => {
                let data = rmp_serde::to_vec(&event).unwrap();
                let _ = socket.send(Message::Binary(data)).await;
            }

            F123Data::Participants(participants) => {
                let data = rmp_serde::to_vec(&participants).unwrap();
                let _ = socket.send(Message::Binary(data)).await;
            }

            F123Data::FinalClassification(classification) => {
                let data = rmp_serde::to_vec(&classification).unwrap();
                let _ = socket.send(Message::Binary(data)).await;
            }
        }

        sleep(Duration::from_millis(700)).await
    }
}
