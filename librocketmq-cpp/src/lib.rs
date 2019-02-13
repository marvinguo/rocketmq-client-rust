//#[link(name = "rocketmq", kind = "static")]
/*
typedef struct CMessage CMessage;
ROCKETMQCLIENT_API CMessage *  CreateMessage(const char *topic);
ROCKETMQCLIENT_API int DestroyMessage(CMessage *msg);
ROCKETMQCLIENT_API int SetMessageTopic(CMessage *msg, const char *topic);
ROCKETMQCLIENT_API int SetMessageTags(CMessage *msg, const char *tags);
ROCKETMQCLIENT_API int SetMessageKeys(CMessage *msg, const char *keys);
ROCKETMQCLIENT_API int SetMessageBody(CMessage *msg, const char *body);
ROCKETMQCLIENT_API int SetByteMessageBody(CMessage *msg, const char *body, int len);
ROCKETMQCLIENT_API int SetMessageProperty(CMessage *msg, const char *key, const char *value);
ROCKETMQCLIENT_API int SetDelayTimeLevel(CMessage *msg, int level);



//typedef struct _CProducer_ _CProducer;
typedef struct CProducer CProducer;
typedef int(*QueueSelectorCallback)(int size, CMessage *msg, void *arg);
typedef void(*CSendSuccessCallback)(CSendResult result);
typedef void(*CSendExceptionCallback)(CMQException e);

ROCKETMQCLIENT_API CProducer *CreateProducer(const char *groupId);
ROCKETMQCLIENT_API int DestroyProducer(CProducer *producer);
ROCKETMQCLIENT_API int StartProducer(CProducer *producer);
ROCKETMQCLIENT_API int ShutdownProducer(CProducer *producer);

ROCKETMQCLIENT_API int SetProducerNameServerAddress(CProducer *producer, const char *namesrv);
ROCKETMQCLIENT_API int SetProducerNameServerDomain(CProducer *producer, const char *domain);
ROCKETMQCLIENT_API int SetProducerGroupName(CProducer *producer, const char *groupName);
ROCKETMQCLIENT_API int SetProducerInstanceName(CProducer *producer, const char *instanceName);
ROCKETMQCLIENT_API int SetProducerSessionCredentials(CProducer *producer, const char *accessKey, const char *secretKey,
                                  const char *onsChannel);
ROCKETMQCLIENT_API int SetProducerLogPath(CProducer *producer, const char *logPath);
ROCKETMQCLIENT_API int SetProducerLogFileNumAndSize(CProducer *producer, int fileNum, long fileSize);
ROCKETMQCLIENT_API int SetProducerLogLevel(CProducer *producer, CLogLevel level);
ROCKETMQCLIENT_API int SetProducerSendMsgTimeout(CProducer *producer, int timeout);
ROCKETMQCLIENT_API int SetProducerCompressLevel(CProducer *producer, int level);
ROCKETMQCLIENT_API int SetProducerMaxMessageSize(CProducer *producer, int size);

ROCKETMQCLIENT_API int SendMessageSync(CProducer *producer, CMessage *msg, CSendResult *result);
ROCKETMQCLIENT_API int SendMessageAsync(CProducer *producer, CMessage *msg, CSendSuccessCallback cSendSuccessCallback , CSendExceptionCallback cSendExceptionCallback);
ROCKETMQCLIENT_API int SendMessageOneway(CProducer *producer,CMessage *msg);

//这里有一个callback，需要用rust来处理
//大部分的函数都需要用到
ROCKETMQCLIENT_API int SendMessageOrderly(CProducer *producer, CMessage *msg, QueueSelectorCallback callback, void *arg, int autoRetryTimes, CSendResult *result);

*/


extern {
    fn CreateMessage();
    fn CreateProducer();
    fn CreatePushConsumer();
    fn DestroyMessage();
    fn DestroyProducer();
    fn DestroyPushConsumer();
    fn GetMessageBody();
    fn GetMessageId();
    fn GetMessageKeys();
    fn GetMessageTags();
    fn GetMessageTopic();
    fn RegisterMessageCallback();
    fn SendMessageSync();
    fn SetByteMessageBody();
    fn SetMessageBody();
    fn SetMessageKeys();
    fn SetMessageTags();
    fn SetProducerCompressLevel();
    fn SetProducerGroupName();
    fn SetProducerInstanceName();
    fn SetProducerLogFileNumAndSize();
    fn SetProducerLogLevel();
    fn SetProducerMaxMessageSize();
    fn SetProducerNameServerAddress();
    fn SetProducerSendMsgTimeout();
    fn SetProducerSessionCredentials();
    fn SetPushConsumerInstanceName();
    fn SetPushConsumerLogFileNumAndSize();
    fn SetPushConsumerLogLevel();
    fn SetPushConsumerMessageBatchMaxSize();
    fn SetPushConsumerNameServerAddress();
    fn SetPushConsumerSessionCredentials();
    fn SetPushConsumerThreadCount();
    fn ShutdownProducer();
    fn ShutdownPushConsumer();
    fn StartProducer();
    fn StartPushConsumer();
    fn Subscribe();
}

