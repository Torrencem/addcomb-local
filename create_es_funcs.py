import re

use_prelude = set()

filtered_code = []

folder_names = ["chapter_" + x for x in ['a', 'b', 'c', 'd', 'e', 'f', 'g']]

for name in folder_names:
    with open("src/comb/" + name + "/mod.rs") as f:
        lines = f.readlines()
        in_test_mod = False
        in_sigma = False
        b_level = 0
        next_define_mod_v = False
        for l in lines:
                # Remove sigma functions
                if "pub fn" in l and "sigma" in l:
                        filtered_code = filtered_code[:-1]
                        in_sigma = True
                        b_level = 1
                        continue
                if "#[cfg(test)]" in l:
                        in_test_mod = True
                        continue
                if in_test_mod:
                        b_level += l.count("{")
                        b_level -= l.count("}")
                        if b_level == 0:
                                in_test_mod = False
                        continue
                if in_sigma:
                        b_level += l.count("{")
                        b_level -= l.count("}")
                        if b_level == 0:
                                in_sigma = False
                        continue
                if "use" in l:
                        use_prelude.add(l)
                        continue

                if re.match(r'pub fn ([\w\_]+)\(', l):
                        next_define_mod_v = True
                
                # Rename functions
                l = re.sub(r'pub fn ([\w\_]+)\(', r'pub fn \1_exact(', l)

                # Remove each_set_exact_zero (temporary)
                l = re.sub(r'each_set_exact_zero', r'each_set_exact', l)

                # Fix group name differences
                l = re.sub(r'n: u32', r'gname: &[u32]', l)

                # Fix each_set_exact
                l = re.sub(r'each_set_exact\([\w\_]+, ([\w\_]+)\)', r'each_set_exact(\1, &mod_v)', l)

                # Fix hfolds
                l = re.sub(r'([\w\_]+).hfold([\w\_]+)\(([^\r\n]+)\)', r'hfold\2(\1, \3)', l)

                # Fix various uses of n
                l = re.sub(r'result.solve\(n\)', r'result.solve(gsize(&mod_v))', l)
                l = re.sub(r'([\w\_]+) == n', r'\1 == gsize(&mod_v)', l)
                l = re.sub(r'([\w\_]+) = n', r'\1 = gsize(&mod_v)', l)
                # As an argument to hfolds
                l = re.sub(r', n\)', r', &mod_v)', l)

                filtered_code.append(l)

                if next_define_mod_v:
                        filtered_code.append("    let mod_v = gname.to_vec();\n")
                        next_define_mod_v = False

# Remove references to fastset
use_prelude = set(filter(lambda line: not ("fastset" in line), use_prelude))

# Import exactset stuff
use_prelude.add("use exactset::*;\n")

# Write the output file
# with open("src/comb/exact_copy_gen.rs", "w") as f:
#         for line in use_prelude:
#                 f.write(line)
#         for line in filtered_code:
#                 f.write(line)
print("This file was only meant to be run once!")