use properties::*;
use std::convert::TryFrom;

#[derive(Debug)]
pub enum DecodeError {
    InvalidPacketType,
    InvalidRemainingLength,
    PacketTooLarge,
    InvalidUtf8,
    InvalidQoS,
    InvalidPropertyId,
    InvalidPropertyForPacket,
    Io(std::io::Error),
}

impl From<std::io::Error> for DecodeError {
    fn from(err: std::io::Error) -> Self {
        DecodeError::Io(err)
    }
}

#[derive(Debug)]
pub enum PacketType {
    Connect,
    ConnectAck,
    Publish,
    PublishAck,
    PublishReceived,
    PublishRelease,
    PublishComplete,
    Subscribe,
    SubscribeAck,
    Unsubscribe,
    UnsubscribeAck,
    PingRequest,
    PingResponse,
    Disconnect,
    Authenticate,
}

impl TryFrom<u8> for PacketType {
    type Error = DecodeError;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            1 => Ok(PacketType::Connect),
            2 => Ok(PacketType::ConnectAck),
            3 => Ok(PacketType::Publish),
            4 => Ok(PacketType::PublishAck),
            5 => Ok(PacketType::PublishReceived),
            6 => Ok(PacketType::PublishRelease),
            7 => Ok(PacketType::PublishComplete),
            8 => Ok(PacketType::Subscribe),
            9 => Ok(PacketType::SubscribeAck),
            10 => Ok(PacketType::Unsubscribe),
            11 => Ok(PacketType::UnsubscribeAck),
            12 => Ok(PacketType::PingRequest),
            13 => Ok(PacketType::PingResponse),
            14 => Ok(PacketType::Disconnect),
            15 => Ok(PacketType::Authenticate),
            _ => Err(DecodeError::InvalidPacketType),
        }
    }
}

#[derive(Debug)]
pub enum QoS {
    AtMostOnce,  // QoS 0
    AtLeastOnce, // QoS 1
    ExactlyOnce, // QoS 2
}

impl TryFrom<u8> for QoS {
    type Error = DecodeError;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0 => Ok(QoS::AtMostOnce),
            1 => Ok(QoS::AtLeastOnce),
            2 => Ok(QoS::ExactlyOnce),
            _ => Err(DecodeError::InvalidQoS),
        }
    }
}

#[derive(Debug)]
pub enum RetainHandling {
    SendAtSubscribeTime,
    SendAtSubscribeTimeIfNonexistent,
    DoNotSend,
}

pub mod properties {
    use super::QoS;
    // Property structs
    #[derive(Debug)]
    pub struct PayloadFormatIndicator(pub u8);
    #[derive(Debug)]
    pub struct MessageExpiryInterval(pub u32);
    #[derive(Debug)]
    pub struct ContentType(pub String);
    #[derive(Debug)]
    pub struct RepsonseTopic(pub String);
    #[derive(Debug)]
    pub struct CorrelationData(pub Vec<u8>);
    #[derive(Debug)]
    pub struct SubscriptionIdentifier(pub u32);
    #[derive(Debug)]
    pub struct SessionExpiryInterval(pub u32);
    #[derive(Debug)]
    pub struct AssignedClientIdentifier(pub String);
    #[derive(Debug)]
    pub struct ServerKeepAlive(pub u16);
    #[derive(Debug)]
    pub struct AuthenticationMethod(pub String);
    #[derive(Debug)]
    pub struct AuthenticationData(pub Vec<u8>);
    #[derive(Debug)]
    pub struct RequestProblemInformation(pub u8);
    #[derive(Debug)]
    pub struct WillDelayInterval(pub u32);
    #[derive(Debug)]
    pub struct RequestResponseInformation(pub u8);
    #[derive(Debug)]
    pub struct ResponseInformation(pub String);
    #[derive(Debug)]
    pub struct ServerReference(pub String);
    #[derive(Debug)]
    pub struct ReasonString(pub String);
    #[derive(Debug)]
    pub struct ReceiveMaximum(pub u16);
    #[derive(Debug)]
    pub struct TopicAliasMaximum(pub u16);
    #[derive(Debug)]
    pub struct TopicAlias(pub u16);
    #[derive(Debug)]
    pub struct MaximumQos(pub QoS);
    #[derive(Debug)]
    pub struct RetainAvailable(pub u8);
    #[derive(Debug)]
    pub struct UserProperty(pub String, pub String);
    #[derive(Debug)]
    pub struct MaximumPacketSize(pub u32);
    #[derive(Debug)]
    pub struct WildcardSubscriptionAvailable(pub u8);
    #[derive(Debug)]
    pub struct SubscriptionIdentifierAvailable(pub u8);
    #[derive(Debug)]
    pub struct SharedSubscriptionAvailable(pub u8);

    #[derive(Debug)]
    pub enum Property {
        PayloadFormatIndicator(PayloadFormatIndicator),
        MessageExpiryInterval(MessageExpiryInterval),
        ContentType(ContentType),
        RepsonseTopic(RepsonseTopic),
        CorrelationData(CorrelationData),
        SubscriptionIdentifier(SubscriptionIdentifier),
        SessionExpiryInterval(SessionExpiryInterval),
        AssignedClientIdentifier(AssignedClientIdentifier),
        ServerKeepAlive(ServerKeepAlive),
        AuthenticationMethod(AuthenticationMethod),
        AuthenticationData(AuthenticationData),
        RequestProblemInformation(RequestProblemInformation),
        WillDelayInterval(WillDelayInterval),
        RequestResponseInformation(RequestResponseInformation),
        ResponseInformation(ResponseInformation),
        ServerReference(ServerReference),
        ReasonString(ReasonString),
        ReceiveMaximum(ReceiveMaximum),
        TopicAliasMaximum(TopicAliasMaximum),
        TopicAlias(TopicAlias),
        MaximumQos(MaximumQos),
        RetainAvailable(RetainAvailable),
        UserProperty(UserProperty),
        MaximumPacketSize(MaximumPacketSize),
        WildcardSubscriptionAvailable(WildcardSubscriptionAvailable),
        SubscriptionIdentifierAvailable(SubscriptionIdentifierAvailable),
        SharedSubscriptionAvailable(SharedSubscriptionAvailable),
    }
}

#[derive(Debug)]
pub enum ConnectReason {
    Success,
    UnspecifiedError,
    MalformedPacket,
    ProtocolError,
    ImplementationSpecificError,
    UnsupportedProtocolVersion,
    ClientIdentifierNotValid,
    BadUserNameOrPassword,
    NotAuthorized,
    ServerUnavailable,
    ServerBusy,
    Banned,
    BadAuthenticationMethod,
    TopicNameInvalid,
    PacketTooLarge,
    QuotaExceeded,
    PayloadFormatInvalid,
    RetainNotSupported,
    QosNotSupported,
    UseAnotherServer,
    ServerMoved,
    ConnectionRateExceeded,
}

#[derive(Debug)]
pub enum PublishAckReason {
    Success,
    NoMatchingSubscribers,
    UnspecifiedError,
    ImplementationSpecificError,
    NotAuthorized,
    TopicNameInvalid,
    PacketIdentifierInUse,
    QuotaExceeded,
    PayloadFormatInvalid,
}

#[derive(Debug)]
pub enum PublishReceivedReason {
    Success,
    NoMatchingSubscribers,
    UnspecifiedError,
    ImplementationSpecificError,
    NotAuthorized,
    TopicNameInvalid,
    PacketIdentifierInUse,
    QuotaExceeded,
    PayloadFormatInvalid,
}

#[derive(Debug)]
pub enum PublishReleaseReason {
    Success,
    PacketIdentifierNotFound,
}

#[derive(Debug)]
pub enum PublishCompleteReason {
    Success,
    PacketIdentifierNotFound,
}

#[derive(Debug)]
pub enum SubscribeAckReason {
    GrantedQoSZero,
    GrantedQoSOne,
    GrantedQoSTwo,
    UnspecifiedError,
    ImplementationSpecificError,
    NotAuthorized,
    TopicFilterInvalid,
    PacketIdentifierInUse,
    QuotaExceeded,
    SharedSubscriptionsNotSupported,
    SubscriptionIdentifiersNotSupported,
    WildcardSubscriptionsNotSupported,
}

#[derive(Debug)]
pub enum UnsubscribeAckReason {
    Success,
    NoSubscriptionExisted,
    UnspecifiedError,
    ImplementationSpecificError,
    NotAuthorized,
    TopicFilterInvalid,
    PacketIdentifierInUse,
}

#[derive(Debug)]
pub enum DisconnectReason {
    NormalDisconnection,
    DisconnectWithWillMessage,
    UnspecifiedError,
    MalformedPacket,
    ProtocolError,
    ImplementationSpecificError,
    NotAuthorized,
    ServerBusy,
    ServerShuttingDown,
    KeepAliveTimeout,
    SessionTakenOver,
    TopicFilterInvalid,
    TopicNameInvalid,
    ReceiveMaximumExceeded,
    TopicAliasInvalid,
    PacketTooLarge,
    MessageRateTooHigh,
    QuotaExceeded,
    AdministrativeAction,
    PayloadFormatInvalid,
    RetainNotSupported,
    QosNotSupported,
    UseAnotherServer,
    ServerMoved,
    SharedSubscriptionNotAvailable,
    ConnectionRateExceeded,
    MaximumConnectTime,
    SubscriptionIdentifiersNotAvailable,
    WildcardSubscriptionsNotAvailable,
}

#[derive(Debug)]
pub enum AuthenticateReason {
    Success,
    ContinueAuthentication,
    ReAuthenticate,
}

// Payloads
#[derive(Debug)]
pub struct FinalWill {
    pub topic: String,
    pub payload: Vec<u8>,
    pub qos: QoS,
    pub should_retain: bool,

    // Properties
    pub will_delay_interval: Option<WillDelayInterval>,
    pub payload_format_indicator: Option<PayloadFormatIndicator>,
    pub message_expiry_interval: Option<MessageExpiryInterval>,
    pub content_type: Option<ContentType>,
    pub response_topic: Option<RepsonseTopic>,
    pub correlation_data: Option<CorrelationData>,
    pub user_properties: Vec<UserProperty>,
}

#[derive(Debug)]
pub struct SubscriptionTopic {
    pub topic: String,
    pub maximum_qos: QoS,
    pub no_local: bool,
    pub retain_as_published: bool,
    pub retain_handling: RetainHandling,
}

// Control Packets
#[derive(Debug)]
pub struct ConnectPacket {
    // Variable Header
    pub protocol_name: String,
    pub protocol_level: u8,
    pub clean_start: bool,
    pub keep_alive: u16,

    // Properties
    pub session_expiry_interval: Option<SessionExpiryInterval>,
    pub receive_maximum: Option<ReceiveMaximum>,
    pub maximum_packet_size: Option<MaximumPacketSize>,
    pub topic_alias_maximum: Option<TopicAliasMaximum>,
    pub request_response_information: Option<RequestResponseInformation>,
    pub request_problem_information: Option<RequestProblemInformation>,
    pub user_properties: Vec<UserProperty>,
    pub authentication_method: Option<AuthenticationMethod>,
    pub authentication_data: Option<AuthenticationData>,

    // Payload
    pub client_id: String,
    pub will: Option<FinalWill>,
    pub user_name: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug)]
pub struct ConnectAckPacket {
    // Variable header
    pub session_present: bool,
    pub reason: ConnectReason,

    // Properties
    pub session_expiry_interval: Option<SessionExpiryInterval>,
    pub receive_maximum: Option<ReceiveMaximum>,
    pub maximum_qos: Option<MaximumQos>,
    pub retain_available: Option<RetainAvailable>,
    pub maximum_packet_size: Option<MaximumPacketSize>,
    pub assigned_client_identifier: Option<AssignedClientIdentifier>,
    pub topic_alias_maximum: Option<TopicAliasMaximum>,
    pub reason_string: Option<ReasonString>,
    pub user_properties: Vec<UserProperty>,
    pub wildcard_subscription_available: Option<WildcardSubscriptionAvailable>,
    pub subscription_identifiers_available: Option<SubscriptionIdentifierAvailable>,
    pub shared_subscription_available: Option<SharedSubscriptionAvailable>,
    pub server_keep_alive: Option<ServerKeepAlive>,
    pub response_information: Option<ResponseInformation>,
    pub server_reference: Option<ServerReference>,
    pub authentication_method: Option<AuthenticationMethod>,
    pub authentication_data: Option<AuthenticationData>,
}

#[derive(Debug)]
pub struct PublishPacket {
    // Variable header
    pub topic_name: String,
    pub packet_id: Option<u16>,

    // Properties
    pub payload_format_indicator: Option<PayloadFormatIndicator>, // TODO(bschwind) - Is this truly optional?
    pub message_expiry_interval: Option<MessageExpiryInterval>,
    pub topic_alias: Option<TopicAlias>,
    pub response_topic: Option<RepsonseTopic>,
    pub correlation_data: Option<CorrelationData>,
    pub user_properties: Vec<UserProperty>,
    pub subscription_identifier: Option<SubscriptionIdentifier>,
    pub content_type: Option<ContentType>,

    // Payload
    pub payload: Vec<u8>,
}

#[derive(Debug)]
pub struct PublishAckPacket {
    // Variable header
    pub packet_id: u16,
    pub reason_code: PublishAckReason,

    // Properties
    pub reason_string: Option<ReasonString>,
    pub user_properties: Vec<UserProperty>,
}

#[derive(Debug)]
pub struct PublishReceivedPacket {
    // Variable header
    pub packet_id: Option<u16>,
    pub reason_code: PublishReceivedReason,

    // Properties
    pub reason_string: Option<ReasonString>,
    pub user_properties: Vec<UserProperty>,
}

#[derive(Debug)]
pub struct PublishReleasePacket {
    // Variable header
    pub packet_id: u16,
    pub reason_code: PublishReleaseReason,

    // Properties
    pub reason_string: Option<ReasonString>,
    pub user_properties: Vec<UserProperty>,
}

#[derive(Debug)]
pub struct PublishCompletePacket {
    // Variable header
    pub packet_id: u16,
    pub reason_code: PublishCompleteReason,

    // Properties
    pub reason_string: Option<ReasonString>,
    pub user_properties: Vec<UserProperty>,
}

#[derive(Debug)]
pub struct SubscribePacket {
    // Variable header
    pub packet_id: u16,

    // Properties
    pub subscription_identifier: Option<SubscriptionIdentifier>,
    pub user_properties: Vec<UserProperty>,

    // Payload
    pub subscription_topics: Vec<SubscriptionTopic>,
}

#[derive(Debug)]
pub struct SubscribeAckPacket {
    // Variable header
    pub packet_id: u16,

    // Properties
    pub reason_string: Option<ReasonString>,
    pub user_properties: Vec<UserProperty>,

    // Payload
    pub reason_codes: Vec<SubscribeAckReason>,
}

#[derive(Debug)]
pub struct UnsubscribePacket {
    // Variable header
    pub packet_id: u16,

    // Properties
    pub user_properties: Vec<UserProperty>,

    // Payload
    pub topics: Vec<String>,
}

#[derive(Debug)]
pub struct UnsubscribeAckPacket {
    // Variable header
    pub packet_id: u16,

    // Properties
    pub reason_string: Option<ReasonString>,
    pub user_properties: Vec<UserProperty>,

    // Payload
    pub reason_codes: Vec<UnsubscribeAckReason>,
}

#[derive(Debug)]
pub struct DisconnectPacket {
    // Variable header
    pub reason_code: DisconnectReason,
    pub packet_id: u16,

    // Properties
    pub session_expiry_interval: Option<SessionExpiryInterval>,
    pub reason_string: Option<ReasonString>,
    pub user_properties: Vec<UserProperty>,
    pub server_reference: Option<ServerReference>,
}

#[derive(Debug)]
pub struct AuthenticatePacket {
    // Variable header
    pub reason_code: AuthenticateReason,

    // Properties
    pub authentication_method: Option<AuthenticationMethod>,
    pub authentication_data: Option<AuthenticationData>,
    pub reason_string: Option<ReasonString>,
    pub user_properties: Vec<UserProperty>,
}

#[derive(Debug)]
pub enum Packet {
    Connect(ConnectPacket),
    ConnectAck(ConnectAckPacket),
    Publish(PublishPacket),
    PublishAck(PublishAckPacket),
    PublishReceived(PublishReceivedPacket),
    PublishRelease(PublishReleasePacket),
    PublishComplete(PublishCompletePacket),
    Subscribe(SubscribePacket),
    SubscribeAck(SubscribeAckPacket),
    Unsubscribe(UnsubscribePacket),
    UnsubscribeAck(UnsubscribeAckPacket),
    PingRequest,
    PingResponse,
    Disconnect(DisconnectPacket),
    Authenticate(AuthenticatePacket),
}
