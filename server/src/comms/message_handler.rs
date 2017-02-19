use std::sync::{Arc, Mutex};

use opcua_core::types::*;
use opcua_core::comms::*;

use types::*;
use server::ServerState;

use services::attribute::*;
use services::discovery::*;
use services::monitored_item::*;
use services::session::*;
use services::subscription::*;
use services::view::*;

/// Processes and dispatches messages for handling
pub struct MessageHandler {
    /// Server state
    server_state: Arc<Mutex<ServerState>>,
    /// Session state
    session_state: Arc<Mutex<SessionState>>,
    /// Attribute service
    attribute_service: AttributeService,
    /// Discovery service
    discovery_service: DiscoveryService,
    /// MonitoredItem service
    monitored_item_service: MonitoredItemService,
    /// Session service
    session_service: SessionService,
    /// Subscription service
    subscription_service: SubscriptionService,
    /// View service
    view_service: ViewService,
}

impl MessageHandler {
    pub fn new(server_state: &Arc<Mutex<ServerState>>, session_state: &Arc<Mutex<SessionState>>) -> MessageHandler {
        MessageHandler {
            server_state: server_state.clone(),
            session_state: session_state.clone(),
            attribute_service: AttributeService::new(),
            discovery_service: DiscoveryService::new(),
            monitored_item_service: MonitoredItemService::new(),
            session_service: SessionService::new(),
            view_service: ViewService::new(),
            subscription_service: SubscriptionService::new(),
        }
    }

    pub fn handle_message(&mut self, message: &SupportedMessage) -> Result<SupportedMessage, &'static StatusCode> {
        let mut server_state = self.server_state.lock().unwrap();
        let mut server_state = &mut server_state;
        let mut session_state = self.session_state.lock().unwrap();
        let mut session_state = &mut session_state;

        let response = match message {
            &SupportedMessage::GetEndpointsRequest(ref request) => {
                self.discovery_service.get_endpoints(server_state, session_state, request)?
            },
            &SupportedMessage::CreateSessionRequest(ref request) => {
                self.session_service.create_session(server_state, session_state, request)?
            },
            &SupportedMessage::CloseSessionRequest(ref request) => {
                self.session_service.close_session(server_state, session_state, request)?
            },
            &SupportedMessage::ActivateSessionRequest(ref request) => {
                self.session_service.activate_session(server_state, session_state, request)?
            },
            &SupportedMessage::CreateSubscriptionRequest(ref request) => {
                self.subscription_service.create_subscription(server_state, session_state, request)?
            },
            &SupportedMessage::ModifySubscriptionRequest(ref request) => {
                self.subscription_service.modify_subscription(server_state, session_state, request)?
            },
            &SupportedMessage::DeleteSubscriptionsRequest(ref request) => {
                self.subscription_service.delete_subscriptions(server_state, session_state, request)?
            }
            &SupportedMessage::PublishRequest(ref request) => {
                self.subscription_service.publish(server_state, session_state, request)?
            },
            &SupportedMessage::BrowseRequest(ref request) => {
                self.view_service.browse(server_state, session_state, request)?
            },
            &SupportedMessage::ReadRequest(ref request) => {
                self.attribute_service.read(server_state, session_state, request)?
            },
            &SupportedMessage::CreateMonitoredItemsRequest(ref request) => {
                self.monitored_item_service.create_monitored_items(server_state, session_state, request)?
            },
            _ => {
                debug!("Message handler does not handle this kind of message");
                return Err(&BAD_SERVICE_UNSUPPORTED);
            }
        };
        Ok(response)
    }
}