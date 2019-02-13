use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/*
//typedef struct _CProducer_ _CProducer;
typedef struct CProducer CProducer;
typedef int(*QueueSelectorCallback)(int size, CMessage *msg, void *arg);
typedef void(*CSendSuccessCallback)(CSendResult result);
typedef void(*CSendExceptionCallback)(CMQException e);




ROCKETMQCLIENT_API int SendMessageSync(CProducer *producer, CMessage *msg, CSendResult *result);
ROCKETMQCLIENT_API int SendMessageAsync(CProducer *producer, CMessage *msg, CSendSuccessCallback cSendSuccessCallback , CSendExceptionCallback cSendExceptionCallback);
ROCKETMQCLIENT_API int SendMessageOneway(CProducer *producer,CMessage *msg);

//这里有一个callback，需要用rust来处理
//大部分的函数都需要用到
ROCKETMQCLIENT_API int SendMessageOrderly(CProducer *producer, CMessage *msg, QueueSelectorCallback callback, void *arg, int autoRetryTimes, CSendResult *result);




//typedef struct _CConsumer_ _CConsumer;
typedef struct CPushConsumer CPushConsumer;

typedef enum E_CConsumeStatus{
    E_CONSUME_SUCCESS = 0,
    E_RECONSUME_LATER = 1
} CConsumeStatus;

typedef int(*MessageCallBack)(CPushConsumer *, CMessageExt *);

ROCKETMQCLIENT_API int SetPushConsumerGroupID(CPushConsumer *consumer, const char *groupId);
ROCKETMQCLIENT_API const char *GetPushConsumerGroupID(CPushConsumer *consumer);
ROCKETMQCLIENT_API int SetPushConsumerNameServerAddress(CPushConsumer *consumer, const char *namesrv);
ROCKETMQCLIENT_API int SetPushConsumerNameServerDomain(CPushConsumer *consumer, const char *domain);
ROCKETMQCLIENT_API int RegisterMessageCallbackOrderly(CPushConsumer *consumer, MessageCallBack pCallback);
ROCKETMQCLIENT_API int RegisterMessageCallback(CPushConsumer *consumer, MessageCallBack pCallback);
ROCKETMQCLIENT_API int UnregisterMessageCallbackOrderly(CPushConsumer *consumer);
ROCKETMQCLIENT_API int UnregisterMessageCallback(CPushConsumer *consumer);
ROCKETMQCLIENT_API int SetPushConsumerThreadCount(CPushConsumer *consumer, int threadCount);
ROCKETMQCLIENT_API int SetPushConsumerMessageBatchMaxSize(CPushConsumer *consumer, int batchSize);
ROCKETMQCLIENT_API int SetPushConsumerInstanceName(CPushConsumer *consumer, const char *instanceName);
ROCKETMQCLIENT_API int SetPushConsumerSessionCredentials(CPushConsumer *consumer, const char *accessKey, const char *secretKey,const char *channel);
ROCKETMQCLIENT_API int SetPushConsumerLogPath(CPushConsumer *consumer, const char *logPath);
ROCKETMQCLIENT_API int SetPushConsumerLogFileNumAndSize(CPushConsumer *consumer, int fileNum, long fileSize);
ROCKETMQCLIENT_API int SetPushConsumerLogLevel(CPushConsumer *consumer, CLogLevel level);
ROCKETMQCLIENT_API int SetPushConsumerMessageModel(CPushConsumer *consumer, CMessageModel messageModel);

typedef struct CMessageExt CMessageExt;

ROCKETMQCLIENT_API const char *GetMessageTopic(CMessageExt *msgExt);
ROCKETMQCLIENT_API const char *GetMessageTags(CMessageExt *msgExt);
ROCKETMQCLIENT_API const char *GetMessageKeys(CMessageExt *msgExt);
ROCKETMQCLIENT_API const char *GetMessageBody(CMessageExt *msgExt);
ROCKETMQCLIENT_API const char *GetMessageProperty(CMessageExt *msgExt, const char *key);
ROCKETMQCLIENT_API const char *GetMessageId(CMessageExt *msgExt);
ROCKETMQCLIENT_API int GetMessageDelayTimeLevel(CMessageExt *msgExt);
ROCKETMQCLIENT_API int GetMessageQueueId(CMessageExt *msgExt);
ROCKETMQCLIENT_API int GetMessageReconsumeTimes(CMessageExt *msgExt);
ROCKETMQCLIENT_API int GetMessageStoreSize(CMessageExt *msgExt);
ROCKETMQCLIENT_API long long GetMessageBornTimestamp(CMessageExt *msgExt);
ROCKETMQCLIENT_API long long GetMessageStoreTimestamp(CMessageExt *msgExt);
ROCKETMQCLIENT_API long long GetMessageQueueOffset(CMessageExt *msgExt);
ROCKETMQCLIENT_API long long GetMessageCommitLogOffset(CMessageExt *msgExt);
ROCKETMQCLIENT_API long long GetMessagePreparedTransactionOffset(CMessageExt *msgExt);
*/
#[repr(C)]
struct CMessage {}

#[repr(C)]
struct CProducer {}

#[repr(C)]
struct CPushConsumer {}

#[repr(C)]
enum CLogLevel {
    E_LOG_LEVEL_FATAL = 1,
    E_LOG_LEVEL_ERROR = 2,
    E_LOG_LEVEL_WARN = 3,
    E_LOG_LEVEL_INFO = 4,
    E_LOG_LEVEL_DEBUG = 5,
    E_LOG_LEVEL_TRACE = 6,
    E_LOG_LEVEL_LEVEL_NUM = 7,
}

#[repr(C)]
enum CSendStatus {
    E_SEND_OK = 0,
    E_SEND_FLUSH_DISK_TIMEOUT = 1,
    E_SEND_FLUSH_SLAVE_TIMEOUT = 2,
    E_SEND_SLAVE_NOT_AVAILABLE = 3,
}

#[repr(C)]
struct CSendResult {
    sendStatus: CSendStatus,
    //char        msgId[MAX_MESSAGE_ID_LENGTH],
    offset: i64,
}

//#[link(name = "rocketmq", kind = "static")]
extern {
    fn CreateMessage(topic: *const c_char) -> *const CMessage;
    fn CreateProducer(groupId: *const c_char) -> *const CProducer;
    fn CreatePushConsumer(groupId: *const c_char) -> *const CPushConsumer;
    fn DestroyMessage(msg: *const CMessage) -> i32;
    fn DestroyProducer(producer: *const CProducer) -> i32;
    fn DestroyPushConsumer(consumer: *const CPushConsumer) -> i32;
    //fn GetMessageBody();
    //fn GetMessageId();
    //fn GetMessageKeys();
    //fn GetMessageTags();
    //fn GetMessageTopic();
    //fn RegisterMessageCallback();
    //fn SendMessageSync();
    fn SetByteMessageBody(msg: *const CMessage, body: *const c_char, len: i32) -> i32;
    fn SetDelayTimeLevel(msg: *const CMessage, level: i32) -> i32;
    fn SetMessageBody(msg: *const CMessage, body: *const c_char) -> i32;
    fn SetMessageProperty(msg: *const CMessage, key: *const c_char, value: *const c_char) -> i32;
    fn SetMessageKeys(msg: *const CMessage, keys: *const c_char) -> i32;
    fn SetMessageTopic(msg: *const CMessage, topic: *const c_char) -> i32;
    fn SetMessageTags(msg: *const CMessage, tags: *const c_char) -> i32;
    fn SetProducerCompressLevel(producer: *const CProducer, level: i32) -> i32;
    fn SetProducerGroupName(producer: *const CProducer, groupName: *const c_char) -> i32;
    fn SetProducerInstanceName(producer: *const CProducer, instanceName: *const c_char) -> i32;
    fn SetProducerLogFileNumAndSize(producer: *const CProducer, fileNum: i32, fileSize: i64) -> i32;
    fn SetProducerLogLevel(producer: *const CProducer, level: CLogLevel) -> i32;
    fn SetProducerLogPath(producer: *const CProducer, logPath: *const c_char) -> i32;
    fn SetProducerMaxMessageSize(producer: *const CProducer, size: i32) -> i32;
    fn SetProducerNameServerAddress(producer: *const CProducer, namesrv: *const c_char) -> i32;
    fn SetProducerNameServerDomain(producer: *const CProducer, domain: *const c_char) -> i32;
    fn SetProducerSendMsgTimeout(producer: *const CProducer, timeout: i32) -> i32;
    fn SetProducerSessionCredentials(producer: *const CProducer, accessKey: *const c_char, secretKey: *const c_char, onsChannel: *const c_char) -> i32;
    //fn SetPushConsumerInstanceName();
    //fn SetPushConsumerLogFileNumAndSize();
    //fn SetPushConsumerLogLevel();
    //fn SetPushConsumerMessageBatchMaxSize();
    //fn SetPushConsumerNameServerAddress();
    //fn SetPushConsumerSessionCredentials();
    //fn SetPushConsumerThreadCount();
    fn ShutdownProducer(producer: *const CProducer) -> i32;
    fn ShutdownPushConsumer(consumer: *const CPushConsumer) -> i32;
    fn StartProducer(producer: *const CProducer) -> i32;
    fn StartPushConsumer(consumer: *const CPushConsumer) -> i32;
    fn Subscribe(consumer: *const CPushConsumer, topic: *const c_char, expression: *const c_char) -> i32;
}

