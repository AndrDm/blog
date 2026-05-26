Arduino Code

```c
#include <Wire.h>
#include <RTClib.h>
#include <Adafruit_GFX.h>
#include <Adafruit_SH110X.h>
#include <ESP8266WiFi.h>
#include <ESP8266WebServer.h>

#include <DHT.h>

#define DHTPIN D3          // Connect DATA to D3 (GPIO0)
#define DHTTYPE DHT11      // Use DHT11 or DHT22

// -------- Taster pin --------
#define BUTTON_PIN D5


bool showTemp = false;     // false = show time, true = show temperature
unsigned long lastDebounce = 0;
const unsigned long debounceDelay = 200;  // ms


// -------- I2C pins --------
#define SDA_PIN D2
#define SCL_PIN D1

// -------- Display config --------
#define OLED_ADDR 0x3C
#define OLED_RST  -1
#define SCREEN_WIDTH  128
#define SCREEN_HEIGHT 64

DHT dht(DHTPIN, DHTTYPE);
RTC_DS1307 rtc;
Adafruit_SH1106G display(SCREEN_WIDTH, SCREEN_HEIGHT, &Wire, OLED_RST);

// -------- WiFi Access Point settings --------
const char* ap_ssid = "ESP-Time-Setup";
const char* ap_password = "12345678";

ESP8266WebServer server(80);

// ------------------ Web Page HTML ------------------
String htmlPage() {
  String s =
  "<!DOCTYPE html><html><head><title>Set RTC Time</title>"
  "<style>"
  "body{font-family:Arial;background:#111;color:#eee;text-align:center;}"
  "input{padding:10px;font-size:18px;margin:8px;}"
  "button{padding:10px 20px;font-size:18px;}"
  "</style></head><body>"
  "<h2>Set Date & Time</h2>"
  "<form action='/set'>"
  "Date: <input type='date' name='date'><br>"
  "Time: <input type='time' name='time' step='1'><br>"
  "<button type='submit'>SET</button>"
  "</form>"
  "</body></html>";
  return s;
}

// ------------------ Handle /set request ------------------
void handleSetTime() {
  if (!server.hasArg("date") || !server.hasArg("time")) {
    server.send(400, "text/plain", "Missing arguments");
    return;
  }

  String date = server.arg("date");   // yyyy-mm-dd
  String time = server.arg("time");   // hh:mm:ss

  // Parse
  int yyyy = date.substring(0,4).toInt();
  int mm   = date.substring(5,7).toInt();
  int dd   = date.substring(8,10).toInt();

  int hh   = time.substring(0,2).toInt();
  int mi   = time.substring(3,5).toInt();
  int ss   = time.substring(6,8).toInt();

  rtc.adjust(DateTime(yyyy, mm, dd, hh, mi, ss));

  server.send(200, "text/plain", "RTC updated!\nYou can close this page.");
}

// ------------------ Setup ------------------
void setup() {
  Serial.begin(115200);
  Wire.begin(SDA_PIN, SCL_PIN);

  // ----- RTC -----
  if (!rtc.begin()) {
    Serial.println("ERROR: DS1307 not found!");
    while (1);
  }

  if (!rtc.isrunning()) {
    Serial.println("RTC not running, setting compile time.");
    rtc.adjust(DateTime(F(__DATE__), F(__TIME__)));
  }

  // ----- OLED -----
  if (!display.begin(OLED_ADDR, true)) {
    Serial.println("ERROR: OLED not found!");
    while (1);
  }


  dht.begin();
  Serial.println("DHT11...");

  pinMode(BUTTON_PIN, INPUT_PULLUP);
  Serial.println("Taster...");

  display.clearDisplay();
  display.setRotation(0);
  display.setTextColor(SH110X_WHITE);
  display.setTextSize(1);

  // Header bar
  display.fillRect(0, 0, SCREEN_WIDTH, 12, SH110X_WHITE);
  display.setCursor(2, 2);
  display.setTextColor(SH110X_BLACK);
  display.print("I2C: D2(SDA), D1(SCK)");
  display.setTextColor(SH110X_WHITE);
  display.display();


  // ----- WiFi Access Point -----
  WiFi.softAP(ap_ssid, ap_password);
  Serial.println("AP started!");
  Serial.print("SSID: "); Serial.println(ap_ssid);
  Serial.print("IP: ");   Serial.println(WiFi.softAPIP());

// Web server routes
server.on("/", []{
  server.send(200, "text/html", htmlPage());
});

server.on("/set", handleSetTime);

server.begin();
Serial.println("Web server running on http://192.168.4.1/");
}

// ------------------ Loop ------------------
void loop() {
  Serial.println("loop begin");
  server.handleClient();
  Serial.println("loop continue");
  
  // ----- Button toggle -----
if (digitalRead(BUTTON_PIN) == LOW) {      // button pressed
  if (millis() - lastDebounce > debounceDelay) {
    showTemp = !showTemp;                  // toggle state
    lastDebounce = millis();
  }
}

  DateTime now = rtc.now();

    Serial.printf("%04d/%02d/%02d %02d:%02d:%02d\n",
                now.year(), now.month(), now.day(),
                now.hour(), now.minute(), now.second());



//DHT
  float h = dht.readHumidity();
  float t = dht.readTemperature(); // Celsius

  
  if (isnan(h) || isnan(t)) {
    Serial.println("Failed to read from DHT11!");
    //return;
  }

  Serial.print("Temp: ");
  Serial.print(t);
  Serial.print(" °C   Humidity: ");
  Serial.print(h);
  Serial.println(" %");


    // Clear content region (keep header)
  display.fillRect(0, 14, SCREEN_WIDTH, SCREEN_HEIGHT - 14, SH110X_BLACK);

  display.setCursor(0, 20);


if (showTemp) {
  // show temperature
  float t = dht.readTemperature();
  float h = dht.readHumidity();

  display.setTextSize(2);
  display.setCursor(0,20);
  display.printf("T: %.1f C\n", t);
  //display.printf("%.1f °C\n", t);
  //display.printf("%.1f %cC", t, 248);
  //display.print("\xB0""C");

  display.setTextSize(1);
  //display.setCursor(0,45);
  display.printf("Feuchtigkeit: %.0f %%", h);

} else {

  display.setTextSize(2);
  display.printf("%02d:%02d:%02d\n", now.hour(), now.minute(), now.second());
  
  display.setTextSize(1);
  display.printf("Date: %02d.%02d.%04d\n", now.day(), now.month(), now.year());
}
  display.drawRect(0, 48, 127, 14, SH110X_WHITE);
  display.setCursor(4, 51);
  //display.print("RTC + OLED OK");
  display.print("WiFi: ESP-Time-Setup");

  display.display();

  delay(250);
}
```

