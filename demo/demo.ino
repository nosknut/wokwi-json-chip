#include <ArduinoJson.h>

void printReceivedJson(DynamicJsonDocument &jsonDoc)
{
    String sender = jsonDoc["sender"].as<String>();
    String receiver = jsonDoc["receiver"].as<String>();
    String topic = jsonDoc["topic"].as<String>();

    int temperature = jsonDoc["temperature"].as<int>();

    Serial.println("Received data:");

    Serial.println("Sender: " + sender);
    Serial.println("Receiver: " + receiver);
    Serial.println("Topic: " + topic);
    Serial.println("Temperature: " + String(temperature));
}

void sendJson() {
    DynamicJsonDocument jsonDoc(500);

    jsonDoc["sender"] = "themometer";
    jsonDoc["receiver"] = "webserver";
    jsonDoc["topic"] = "thermometer-sensor-data";
    jsonDoc["temperature"] = 2;

    String jsonText;
    serializeJson(jsonDoc, jsonText);

    Serial1.print(jsonText);

    // Alternatively this can be done without the string
    // serializeJson(jsonDoc, Serial1);
}

void setup()
{
    Serial.begin(115200);
    Serial1.begin(115200);

    sendJson();
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

        printReceivedJson(jsonDoc);
    }
}

void loop()
{
    readSerial();
}
