enum ConsumeType {
    CONSUME_ACTIVELY,
    CONSUME_PASSIVELY,
}

//<!***************************************************************************
enum ConsumeFromWhere {
    CONSUME_FROM_LAST_OFFSET,
    CONSUME_FROM_LAST_OFFSET_AND_FROM_MIN_WHEN_BOOT_FIRST,
    CONSUME_FROM_MIN_OFFSET,
    CONSUME_FROM_MAX_OFFSET,
    CONSUME_FROM_FIRST_OFFSET,
    CONSUME_FROM_TIMESTAMP,
}