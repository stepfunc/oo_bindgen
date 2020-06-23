#include "dnp3rs.h"

#include <stdio.h>
#include <string.h>

void print_qualifier(qualifier_code_t qualifier)
{
    switch(qualifier)
    {
    case QualifierCode_Range8:
        printf("Range8");
        break;
    case QualifierCode_Range16:
        printf("Range16");
        break;
    case QualifierCode_AllObjects:
        printf("AllObjects");
        break;
    case QualifierCode_Count8:
        printf("Count8");
        break;
    case QualifierCode_Count16:
        printf("Count16");
        break;
    case QualifierCode_CountAndPrefix8:
        printf("CountAndPrefix8");
        break;
    case QualifierCode_CountAndPrefix16:
        printf("CountAndPrefix16");
        break;
    case QualifierCode_FreeFormat16:
        printf("FreeFormat16");
        break;
    default:
        printf("Unknown (%u)", qualifier);
        break;
    }
}

void print_variation(variation_t variation)
{
    switch(variation)
    {
    case Variation_Group1Var0:
        printf("Group1Var0");
        break;
    case Variation_Group1Var1:
        printf("Group1Var1");
        break;
    case Variation_Group1Var2:
        printf("Group1Var2");
        break;
    case Variation_Group2Var0:
        printf("Group2Var0");
        break;
    case Variation_Group2Var1:
        printf("Group2Var1");
        break;
    case Variation_Group2Var2:
        printf("Group2Var2");
        break;
    case Variation_Group2Var3:
        printf("Group2Var3");
        break;
    case Variation_Group3Var0:
        printf("Group3Var0");
        break;
    case Variation_Group3Var1:
        printf("Group3Var1");
        break;
    case Variation_Group3Var2:
        printf("Group3Var2");
        break;
    case Variation_Group4Var0:
        printf("Group4Var0");
        break;
    case Variation_Group4Var1:
        printf("Group4Var1");
        break;
    case Variation_Group4Var2:
        printf("Group4Var2");
        break;
    case Variation_Group4Var3:
        printf("Group4Var3");
        break;
    case Variation_Group10Var0:
        printf("Group10Var0");
        break;
    case Variation_Group10Var1:
        printf("Group10Var1");
        break;
    case Variation_Group10Var2:
        printf("Group10Var2");
        break;
    case Variation_Group11Var0:
        printf("Group11Var0");
        break;
    case Variation_Group11Var1:
        printf("Group11Var1");
        break;
    case Variation_Group11Var2:
        printf("Group11Var2");
        break;
    case Variation_Group12Var0:
        printf("Group12Var0");
        break;
    case Variation_Group12Var1:
        printf("Group12Var1");
        break;
    case Variation_Group13Var1:
        printf("Group13Var1");
        break;
    case Variation_Group13Var2:
        printf("Group13Var2");
        break;
    case Variation_Group20Var0:
        printf("Group20Var0");
        break;
    case Variation_Group20Var1:
        printf("Group20Var1");
        break;
    case Variation_Group20Var2:
        printf("Group20Var2");
        break;
    case Variation_Group20Var5:
        printf("Group20Var5");
        break;
    case Variation_Group20Var6:
        printf("Group20Var6");
        break;
    case Variation_Group21Var0:
        printf("Group21Var0");
        break;
    case Variation_Group21Var1:
        printf("Group21Var1");
        break;
    case Variation_Group21Var2:
        printf("Group21Var2");
        break;
    case Variation_Group21Var5:
        printf("Group21Var5");
        break;
    case Variation_Group21Var6:
        printf("Group21Var6");
        break;
    case Variation_Group21Var9:
        printf("Group21Var9");
        break;
    case Variation_Group21Var10:
        printf("Group21Var10");
        break;
    case Variation_Group22Var0:
        printf("Group22Var0");
        break;
    case Variation_Group22Var1:
        printf("Group22Var1");
        break;
    case Variation_Group22Var2:
        printf("Group22Var2");
        break;
    case Variation_Group22Var5:
        printf("Group22Var5");
        break;
    case Variation_Group22Var6:
        printf("Group22Var6");
        break;
    case Variation_Group23Var0:
        printf("Group23Var0");
        break;
    case Variation_Group23Var1:
        printf("Group23Var1");
        break;
    case Variation_Group23Var2:
        printf("Group23Var2");
        break;
    case Variation_Group23Var5:
        printf("Group23Var5");
        break;
    case Variation_Group23Var6:
        printf("Group23Var6");
        break;
    case Variation_Group30Var0:
        printf("Group30Var0");
        break;
    case Variation_Group30Var1:
        printf("Group30Var1");
        break;
    case Variation_Group30Var2:
        printf("Group30Var2");
        break;
    case Variation_Group30Var3:
        printf("Group30Var3");
        break;
    case Variation_Group30Var4:
        printf("Group30Var4");
        break;
    case Variation_Group30Var5:
        printf("Group30Var5");
        break;
    case Variation_Group30Var6:
        printf("Group30Var6");
        break;
    case Variation_Group32Var0:
        printf("Group32Var0");
        break;
    case Variation_Group32Var1:
        printf("Group32Var1");
        break;
    case Variation_Group32Var2:
        printf("Group32Var2");
        break;
    case Variation_Group32Var3:
        printf("Group32Var3");
        break;
    case Variation_Group32Var4:
        printf("Group32Var4");
        break;
    case Variation_Group32Var5:
        printf("Group32Var5");
        break;
    case Variation_Group32Var6:
        printf("Group32Var6");
        break;
    case Variation_Group32Var7:
        printf("Group32Var7");
        break;
    case Variation_Group32Var8:
        printf("Group32Var8");
        break;
    case Variation_Group40Var0:
        printf("Group40Var0");
        break;
    case Variation_Group40Var1:
        printf("Group40Var1");
        break;
    case Variation_Group40Var2:
        printf("Group40Var2");
        break;
    case Variation_Group40Var3:
        printf("Group40Var3");
        break;
    case Variation_Group40Var4:
        printf("Group40Var4");
        break;
    case Variation_Group41Var0:
        printf("Group41Var0");
        break;
    case Variation_Group41Var1:
        printf("Group41Var1");
        break;
    case Variation_Group41Var2:
        printf("Group41Var2");
        break;
    case Variation_Group41Var3:
        printf("Group41Var3");
        break;
    case Variation_Group41Var4:
        printf("Group41Var4");
        break;
    case Variation_Group42Var0:
        printf("Group42Var0");
        break;
    case Variation_Group42Var1:
        printf("Group42Var1");
        break;
    case Variation_Group42Var2:
        printf("Group42Var2");
        break;
    case Variation_Group42Var3:
        printf("Group42Var3");
        break;
    case Variation_Group42Var4:
        printf("Group42Var4");
        break;
    case Variation_Group42Var5:
        printf("Group42Var5");
        break;
    case Variation_Group42Var6:
        printf("Group42Var6");
        break;
    case Variation_Group42Var7:
        printf("Group42Var7");
        break;
    case Variation_Group42Var8:
        printf("Group42Var8");
        break;
    case Variation_Group43Var1:
        printf("Group43Var1");
        break;
    case Variation_Group43Var2:
        printf("Group43Var2");
        break;
    case Variation_Group43Var3:
        printf("Group43Var3");
        break;
    case Variation_Group43Var4:
        printf("Group43Var4");
        break;
    case Variation_Group43Var5:
        printf("Group43Var5");
        break;
    case Variation_Group43Var6:
        printf("Group43Var6");
        break;
    case Variation_Group43Var7:
        printf("Group43Var7");
        break;
    case Variation_Group43Var8:
        printf("Group43Var8");
        break;
    case Variation_Group50Var1:
        printf("Group50Var1");
        break;
    case Variation_Group50Var3:
        printf("Group50Var3");
        break;
    case Variation_Group50Var4:
        printf("Group50Var4");
        break;
    case Variation_Group51Var1:
        printf("Group51Var1");
        break;
    case Variation_Group51Var2:
        printf("Group51Var2");
        break;
    case Variation_Group52Var1:
        printf("Group52Var1");
        break;
    case Variation_Group52Var2:
        printf("Group52Var2");
        break;
    case Variation_Group60Var1:
        printf("Group60Var1");
        break;
    case Variation_Group60Var2:
        printf("Group60Var2");
        break;
    case Variation_Group60Var3:
        printf("Group60Var3");
        break;
    case Variation_Group60Var4:
        printf("Group60Var4");
        break;
    case Variation_Group80Var1:
        printf("Group80Var1");
        break;
    case Variation_Group110:
        printf("Group110");
        break;
    case Variation_Group111:
        printf("Group111");
        break;
    case Variation_Group112:
        printf("Group112");
        break;
    case Variation_Group113:
        printf("Group113");
        break;
    default:
        printf("Unknown (%u)", variation);
        break;
    }
}

// Logger callback
void on_log_message(log_level_t level, char* msg, void* arg)
{
    switch(level)
    {
    case LogLevel_Trace:
        printf("TRACE: %s\n", msg);
        break;
    case LogLevel_Debug:
        printf("DEBUG: %s\n", msg);
        break;
    case LogLevel_Info:
        printf("INFO: %s\n", msg);
        break;
    case LogLevel_Warn:
        printf("WARN: %s\n", msg);
        break;
    case LogLevel_Error:
        printf("ERROR: %s\n", msg);
        break;
    default:
        printf("UNKNOWN: %s\n", msg);
        break;
    }
}

// ClientState listener callback
void client_state_on_change(client_state_t state, void* arg)
{
    switch(state)
    {
    case ClientState_Connecting:
        printf("ClientState = Connecting\n");
        break;
    case ClientState_Connected:
        printf("ClientState = Connected\n");
        break;
    case ClientState_WaitAfterFailedConnect:
        printf("ClientState = WaitAfterFailedConnect\n");
        break;
    case ClientState_WaitAfterDisconnect:
        printf("ClientState: WaitAfterDisconnect\n");
        break;
    case ClientState_Shutdown:
        printf("ClientState: Shutdown\n");
        break;
    default:
        printf("ClientState: Unknown(%u)\n", state);
        break;
    }
}

// ReadHandler callbacks
void begin_fragment(response_header_t header, void* arg)
{
    printf("Beginning fragment (broadcast: %u)\n", iin1_is_set(&header.iin.iin1, IIN1Flag_Broadcast));
}

void end_fragment(response_header_t header, void* arg)
{
    printf("End fragment\n");
}

void handle_binary(header_info_t info, binary_iterator_t* it, void* arg)
{
    printf("Binaries:\n");
    printf("Qualifier: ");
    print_qualifier(info.qualifier);
    printf("\n");
    printf("Variation: ");
    print_variation(info.variation);
    printf("\n");

    binary_t* value = binary_next(it);
    while(value != NULL)
    {
        printf("BI %u: Value=%u Flags=0x%02X Time=%llu\n", value->index, value->value, value->flags.value, value->time.value);
        value = binary_next(it);
    }
}

void handle_double_bit_binary(header_info_t info, double_bit_binary_iterator_t* it, void* arg)
{
    printf("Double Bit Binaries:\n");
    printf("Qualifier: ");
    print_qualifier(info.qualifier);
    printf("\n");
    printf("Variation: ");
    print_variation(info.variation);
    printf("\n");

    double_bit_binary_t* value = doublebitbinary_next(it);
    while(value != NULL)
    {
        printf("DBBI %u: Value=%X Flags=0x%02X Time=%llu\n", value->index, value->value, value->flags.value, value->time.value);
        value = doublebitbinary_next(it);
    }
}

void handle_binary_output_status(header_info_t info, binary_output_status_iterator_t* it, void* arg)
{
    printf("Binary Output Statuses:\n");
    printf("Qualifier: ");
    print_qualifier(info.qualifier);
    printf("\n");
    printf("Variation: ");
    print_variation(info.variation);
    printf("\n");

    binary_output_status_t* value = binaryoutputstatus_next(it);
    while(value != NULL)
    {
        printf("BOS %u: Value=%u Flags=0x%02X Time=%llu\n", value->index, value->value, value->flags.value, value->time.value);
        value = binaryoutputstatus_next(it);
    }
}

void handle_counter(header_info_t info, counter_iterator_t* it, void* arg)
{
    printf("Counters:\n");
    printf("Qualifier: ");
    print_qualifier(info.qualifier);
    printf("\n");
    printf("Variation: ");
    print_variation(info.variation);
    printf("\n");

    counter_t* value = counter_next(it);
    while(value != NULL)
    {
        printf("Counter %u: Value=%u Flags=0x%02X Time=%llu\n", value->index, value->value, value->flags.value, value->time.value);
        value = counter_next(it);
    }
}

void handle_frozen_counter(header_info_t info, frozen_counter_iterator_t* it, void* arg)
{
    printf("Frozen Counters:\n");
    printf("Qualifier: ");
    print_qualifier(info.qualifier);
    printf("\n");
    printf("Variation: ");
    print_variation(info.variation);
    printf("\n");

    frozen_counter_t* value = frozencounter_next(it);
    while(value != NULL)
    {
        printf("Frozen Counter %u: Value=%u Flags=0x%02X Time=%llu\n", value->index, value->value, value->flags.value, value->time.value);
        value = frozencounter_next(it);
    }
}

void handle_analog(header_info_t info, analog_iterator_t* it, void* arg)
{
    printf("Analogs:\n");
    printf("Qualifier: ");
    print_qualifier(info.qualifier);
    printf("\n");
    printf("Variation: ");
    print_variation(info.variation);
    printf("\n");

    analog_t* value = analog_next(it);
    while(value != NULL)
    {
        printf("AI %u: Value=%f Flags=0x%02X Time=%llu\n", value->index, value->value, value->flags.value, value->time.value);
        value = analog_next(it);
    }
}

void handle_analog_output_status(header_info_t info, analog_output_status_iterator_t* it, void* arg)
{
    printf("Analog Output Statuses:\n");
    printf("Qualifier: ");
    print_qualifier(info.qualifier);
    printf("\n");
    printf("Variation: ");
    print_variation(info.variation);
    printf("\n");

    analog_output_status_t* value = analogoutputstatus_next(it);
    while(value != NULL)
    {
        printf("AOS %u: Value=%f Flags=0x%02X Time=%llu\n", value->index, value->value, value->flags.value, value->time.value);
        value = analogoutputstatus_next(it);
    }
}

// Single read callback
void on_read_complete(read_result_t result, void* arg)
{
    switch(result)
    {
    case ReadResult_Success:
        printf("ReadResult = Success\n");
        break;
    case ReadResult_TaskError:
        printf("ReadResult = TaskError\n");
        break;
    default:
        printf("ReadResult: Unknown(%u)\n", result);
        break;
    }
}

// Command callback
void on_command_complete(command_result_t result, void* arg)
{
    switch(result)
    {
    case CommandResult_Success:
        printf("CommandResult: Success\n");
        break;
    case CommandResult_TaskError:
        printf("CommandResult: TaskError\n");
        break;
    case CommandResult_BadStatus:
        printf("CommandResult: BadStatus\n");
        break;
    case CommandResult_HeaderCountMismatch:
        printf("CommandResult: HeaderCountMismatch\n");
        break;
    case CommandResult_HeaderTypeMismatch:
        printf("CommandResult: HeaderTypeMismatch\n");
        break;
    case CommandResult_ObjectCountMismatch:
        printf("CommandResult: ObjectCountMismatch\n");
        break;
    case CommandResult_ObjectValueMismatch:
        printf("CommandResult: ObjectValueMismatch\n");
        break;
    default:
        printf("CommandResult: Unknown(%u)\n", result);
        break;
    }
}

// Timesync callback
void on_timesync_complete(time_sync_result_t result, void* arg)
{
    switch(result)
    {
    case TimeSyncResult_Success:
        printf("TimeSyncResult: Success\n");
        break;
    case TimeSyncResult_TaskError:
        printf("TimeSyncResult: TaskError\n");
        break;
    case TimeSyncResult_ClockRollback:
        printf("TimeSyncResult: ClockRollback\n");
        break;
    case TimeSyncResult_SystemTimeNotUnix:
        printf("TimeSyncResult: SystemTimeNotUnix\n");
        break;
    case TimeSyncResult_BadOutstationTimeDelay:
        printf("TimeSyncResult: BadOutstationTimeDelay\n");
        break;
    case TimeSyncResult_Overflow:
        printf("TimeSyncResult: Overflow\n");
        break;
    case TimeSyncResult_StillNeedsTime:
        printf("TimeSyncResult: StillNeedsTime\n");
        break;
    case TimeSyncResult_IINError:
        printf("TimeSyncResult: IINError\n");
        break;
    default:
        printf("TimeSyncResult: Unknown(%u)\n", result);
        break;
    }
}

int main()
{
    // Setup logging
    logger_t logger =
    {
        .on_message = &on_log_message,
        .arg = NULL,
    };
    logging_set_log_level(LogLevel_Info);
    logging_set_callback(logger);

    // Create runtime
    runtime_config_t runtime_config =
    {
        .num_core_threads = 4,
    };
    runtime_t* runtime = runtime_new(&runtime_config);

    // Create the master
    reconnect_strategy_t strategy =
    {
        .min_delay = 100,
        .max_delay = 5000,
    };
    client_state_listener_t listener =
    {
        .on_change = &client_state_on_change,
        .arg = NULL,
    };
    master_t* master = runtime_add_master_tcp(
        runtime,
        1,
        DecodeLogLevel_ObjectValues,
        strategy,
        5000,
        "127.0.0.1:20000",
        listener
    );

    // Create the association
    read_handler_t read_handler =
    {
        .begin_fragment = &begin_fragment,
        .end_fragment = &end_fragment,
        .handle_binary = &handle_binary,
        .handle_double_bit_binary = &handle_double_bit_binary,
        .handle_binary_output_status = &handle_binary_output_status,
        .handle_counter = &handle_counter,
        .handle_frozen_counter = &handle_frozen_counter,
        .handle_analog = &handle_analog,
        .handle_analog_output_status = &handle_analog_output_status,
        .arg = NULL,
    };
    association_configuration_t association_config =
    {
        .disable_unsol_classes =
        {
            .class1 = true,
            .class2 = true,
            .class3 = true,
        },
        .enable_unsol_classes =
        {
            .class1 = true,
            .class2 = true,
            .class3 = true,
        },
        .auto_time_sync = AutoTimeSync_LAN,
    };
    association_handlers_t association_handlers =
    {
        .integrity_handler = read_handler,
        .unsolicited_handler = read_handler,
        .default_poll_handler = read_handler,
    };
    association_t* association = master_add_association(
        master,
        1024,
        association_config,
        association_handlers
    );

    // Add an event poll
    request_t* poll_request = request_new_class(false, true, true, true);
    poll_t* poll = association_add_poll(association, poll_request, 5000);
    request_destroy(poll_request);

    char cbuf[5];
    while(true)
    {
        fgets(cbuf, 5, stdin);

        if(strcmp(cbuf, "x\n") == 0)
        {
            goto cleanup;
        }
        else if(strcmp(cbuf, "dln\n") == 0)
        {
            master_set_decode_log_level(master, DecodeLogLevel_Nothing);
        }
        else if(strcmp(cbuf, "dlv\n") == 0)
        {
            master_set_decode_log_level(master, DecodeLogLevel_ObjectValues);
        }
        else if(strcmp(cbuf, "rao\n") == 0)
        {
            request_t* request = request_new();
            request_add_all_objects_header(request, Variation_Group40Var0);

            read_task_callback_t cb =
            {
                .on_complete = &on_read_complete,
                .arg = NULL,
            };
            association_read(association, request, cb);

            request_destroy(request);
        }
        else if(strcmp(cbuf, "rmo\n") == 0)
        {
            request_t* request = request_new();
            request_add_all_objects_header(request, Variation_Group10Var0);
            request_add_all_objects_header(request, Variation_Group40Var0);

            read_task_callback_t cb =
            {
                .on_complete = &on_read_complete,
                .arg = NULL,
            };
            association_read(association, request, cb);

            request_destroy(request);
        }
        else if(strcmp(cbuf, "cmd\n") == 0)
        {
            command_t* command = command_new();
            g12v1_t g12v1 =
            {
                .code =
                {
                    .tcc = TripCloseCode_Nul,
                    .clear = false,
                    .queue = false,
                    .op_type = OpType_LatchOn,
                },
                .count = 1,
                .on_time = 1000,
                .off_time = 1000,
            };
            command_add_u16_g12v1(command, 3, g12v1);

            command_task_callback_t cb =
            {
                .on_complete = &on_command_complete,
                .arg = NULL,
            };

            association_operate(
                association,
                CommandMode_SelectBeforeOperate,
                command,
                cb
            );

            command_destroy(command);
        }
        else if(strcmp(cbuf, "evt\n") == 0)
        {
            poll_demand(poll);
        }
        else if(strcmp(cbuf, "lts\n") == 0)
        {
            time_sync_task_callback_t cb =
            {
                .on_complete = &on_timesync_complete,
                .arg = NULL,
            };
            association_perform_time_sync(association, TimeSyncMode_LAN, cb);
        }
        else if(strcmp(cbuf, "nts\n") == 0)
        {
            time_sync_task_callback_t cb =
            {
                .on_complete = &on_timesync_complete,
                .arg = NULL,
            };
            association_perform_time_sync(association, TimeSyncMode_NonLAN, cb);
        }
        else
        {
            printf("Unknown command\n");
        }
    }

    // Cleanup
cleanup:
    poll_destroy(poll);
    association_destroy(association);
    master_destroy(master);
    runtime_destroy(runtime);

    return 0;
}
