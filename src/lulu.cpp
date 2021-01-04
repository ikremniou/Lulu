#include "sciter-x-window.hpp"
#include "sciter-x.h"

#include <functional>

class LuluMainFrame : public sciter::window {
 public:
  LuluMainFrame() : window(SW_TITLEBAR | SW_RESIZEABLE | SW_CONTROLS | SW_MAIN | SW_ENABLE_DEBUG) {}

  SOM_PASSPORT_BEGIN(LuluMainFrame)
  SOM_FUNCS(
      SOM_FUNC(NativeMessage))
  SOM_PASSPORT_END

  sciter::string NativeMessage() { return L"Hello C++ World"; }
};

#include "resources.cpp"// resources packaged into binary blob.

int uimain(const std::function<int()> event_loop) {
  sciter::archive::instance().open(aux::elements_of(resources));
  sciter::om::hasset<LuluMainFrame> lulu_main_frame = new LuluMainFrame();

  lulu_main_frame->load(L"this://app/index.html");
  lulu_main_frame->expand();

  return event_loop();
}