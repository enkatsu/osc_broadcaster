import oscP5.*;
import netP5.*;


OscP5 oscP5;
NetAddress myBroadcastLocation;

void setup() {
  size(400, 400);
  frameRate(25);
  oscP5 = new OscP5(this, "127.0.0.1", 12000, OscP5.UDP);
  myBroadcastLocation = new NetAddress("127.0.0.1", 32000);
}


void draw() {
  background(0);
}


void mousePressed() {
  OscMessage myOscMessage = new OscMessage("/mouse/pressed");
  myOscMessage.add(mouseX);
  myOscMessage.add(mouseY);
  oscP5.send(myOscMessage, myBroadcastLocation);
}


void keyPressed() {
  OscMessage m;
  switch(key) {
    case('c'):
      m = new OscMessage("/server/connect", new Object[0]);
      oscP5.send(m, myBroadcastLocation);
      break;
    case('d'):
      m = new OscMessage("/server/disconnect", new Object[0]);
      oscP5.send(m, myBroadcastLocation);
      break;
  }
}

void oscEvent(OscMessage theOscMessage) {
  String addr = theOscMessage.addrPattern();
  String typetag = theOscMessage.typetag();
  println("### received an osc message with addrpattern " + addr + " and typetag " + typetag);
  theOscMessage.print();
}
