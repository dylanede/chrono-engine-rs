#include "chrono/physics/ChSystem.h"
#include "chrono/physics/ChBody.h"

extern "C" {
  chrono::ChSystem* new_system(unsigned int max_objects, double scene_size, uint init_sys) {
    return new chrono::ChSystem(max_objects, scene_size, init_sys != 0);
  }
  void delete_system(chrono::ChSystem* system) {
    delete system;
  }
  void system_set_step(chrono::ChSystem* self, double step) {
    self->SetStep(step);
  }
  double system_get_step(chrono::ChSystem* self) {
    return self->GetStep();
  }
}

static_assert(sizeof(std::shared_ptr<chrono::ChBody>) == 2 * sizeof(size_t), "shared_ptr size incorrect");
extern "C" {
  void delete_body(chrono::ChBody* self) {
    delete self;
  }
  std::shared_ptr<chrono::ChBody> make_shared_body() {
    return std::make_shared<chrono::ChBody>();
  }
  std::shared_ptr<chrono::ChBody> body_into_shared(chrono::ChBody* self) {
    return std::shared_ptr<chrono::ChBody>(self);
  }
  std::shared_ptr<chrono::ChBody> shared_body_clone(std::shared_ptr<chrono::ChBody> const* self) {
    return *self;
  }
  void drop_shared_body(std::shared_ptr<chrono::ChBody>* self) {
    self->~shared_ptr();
  }
  chrono::ChBody* const shared_body_deref(std::shared_ptr<chrono::ChBody> const* self) {
    return self->get();
  }
}
