## Hint 3: Algorithm Outline

**new(name, role, level) -> Self:**
```
Step 1: Convert name and role using .into() or just use them directly
Step 2: Construct and return CrewMember { name, role, level }
```

**promote(&mut self, new_role) -> ():**
```
Step 1: Update self.role = new_role.into()
Step 2: Increment self.level by 1
Step 3: Cap self.level at 99 using min() or a conditional
```

**badge(&self) -> String:**
```
Step 1: Use format!() to build "[L##] Name — Role"
Step 2: Format level with {:02} (zero-padded to 2 digits)
Step 3: Return the formatted String
```

**Spoiler threshold:** High—gives the structure and format strings, but not the exact Rust code.
