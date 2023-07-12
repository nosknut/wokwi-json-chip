#include "AsyncMacros.h"
#include <ArduinoJson.h>

DynamicJsonDocument servoStatusJsonDoc(500);

DynamicJsonDocument getServoInitCommend()
{
    DynamicJsonDocument jsonDoc(500);
    jsonDoc["topic"] = "servo/init";
    return jsonDoc;
}

DynamicJsonDocument getServoPositionCommend(int position)
{
    DynamicJsonDocument jsonDoc(500);
    jsonDoc["topic"] = "servo/target-position";
    jsonDoc["position"] = position;
    return jsonDoc;
}

void sendJson(DynamicJsonDocument jsonDoc)
{
    serializeJson(jsonDoc, Serial1);
}

void processReceivedJson(DynamicJsonDocument &jsonDoc)
{
    String topic = jsonDoc["topic"].as<String>();

    if (topic == "servo/status")
    {
        servoStatusJsonDoc = jsonDoc;
    }
}

void setup()
{
    Serial.begin(115200);
    Serial1.begin(115200);
}

void readSerial()
{
    // Is there data on the serial port?
    if (Serial1.available() > 0)
    {
        // Create a variable of the data type DynamicJsonDocument that has a
        // capacity of 500 bytes (probably enough for our purposes)
        DynamicJsonDocument jsonDoc(500);

        // Read the string coming from the "Serial" port, find the variables
        // and save them in the jsonDoc variable
        DeserializationError error = deserializeJson(jsonDoc, Serial1);

        // If there is an error, print it out
        // and return from (stop running) the function
        if (error != DeserializationError::Ok)
        {
            Serial.print(F("deserializeJson() failed with the error message: "));
            Serial.println(error.c_str());
            return;
        }

        processReceivedJson(jsonDoc);
    }
}

bool initServoSequence()
{
    asyncBegin({
        asyncRun({
            Serial.println("Initializing servo");
            sendJson(getServoInitCommend());
        });

        // Wait for the servo to respond with a status message
        asyncWhile(!servoStatusJsonDoc.containsKey("position"), {
            asyncDelay(1000);
        });

        asyncRun({
            return true;
        });
    });
    return false;
}

bool servoGoToPositionSequence(int position)
{
    asyncBegin({
        asyncRun({
            Serial.println("Moving servo to " + String(position) + " degrees");
            sendJson(getServoPositionCommend(position));
        });

        // Wait for the servo to reach the target position
        asyncWhile(servoStatusJsonDoc["position"].as<int>() != position, {
            asyncDelay(1000);
        });

        asyncRun({
            return true;
        });
    });
    return false;
}

void updateSequence()
{
    asyncBegin({
        asyncWhile(!initServoSequence(), {});

        asyncWhile(true, {
            asyncWhile(!servoGoToPositionSequence(0), {});
            asyncWhile(!servoGoToPositionSequence(180), {});
        });
    });
}

void loop()
{
    readSerial();

    updateSequence();
}
