import QtQuick 2.2
import TestModule 1.0

Item {
  TestRsObject{
    id: rsRegistered

    name: "Oak"
  }
  Component.onCompleted: {
    console.log(rsRegistered.name)
    rsRegistered.name_changed.connect(jsNameChange);
    rsRegistered.name = "This is working"
    console.log(rsRegistered.name)
    rsRegistered.assure_everything_okay()
    console.log(TestRsSingleton.temp)
  }
  function jsNameChange() {
    console.log("Name was changed to " + rsRegistered.name);
  }
}
