import boolrule
import time

# quick benchmark that can be compared to the release build of coolrule

t = time.time()
for _ in range(0, 1000):
    boolrule.BoolRule("true == false or (1, 2, 3) ⊆ (1, 2, 3)").test()
print(time.time() - t)
