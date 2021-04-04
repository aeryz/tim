
<h1 align="center">

Testing Is a Must

 ![MIT][logo-mit] [![TWEET][logo-tw]][link-tw]

[logo-mit]: https://img.shields.io/badge/license-MIT-blue.svg
[logo-tw]: https://img.shields.io/twitter/follow/aeryz2?label=follow&style=social
[link-tw]: https://twitter.com/intent/follow?screen_name=aeryz2

</h1>

[mit]: https://img.shields.io/badge/license-MIT-blue.svg
[tw]: https://twitter.com/intent/follow?screen_name=aeryz2

<h4 align="center">Easy to use testing framework for C.</h4>

## Get Started (quick)

1. Make an awesome C project.

2. Put `tim.h` and `tim.c` to your project.

3. Write some tim-tests in files that end with `_test.c`.
```c
TIM_TEST(cool_stuff) {
    TIM_ASSERT_EQ(true, false);
    TIM_SUCCESS();
}
```
4. Create a `CMakeLists.txt` in the project root.

5. Write some cmake stuff. (Don't forget to add `tim.c` to your sources)
```cmake
if (DEFINED TIM_TEST_BUILD)
    add_library(my_awesome_lib SHARED foo_test.c bar_test.c tim.c)
    # These will hopefully won't be necessary in the future :)
    set_target_properties(rtw PROPERTIES PREFIX "")
    set_target_properties(rtw PROPERTIES SUFFIX "")
    set_target_properties(rtw PROPERTIES OUTPUT_NAME "tim-test-lib")
endif()
```

6. Run TIM.
```bash
tim -p /path/to/my/project
```

## Installation

`tim` is not yet published to [crates.io](crates.io). So it is necessary to clone the repository.

```bash
git clone https://github.com/aeryz/tim
cargo install --path /path/to/tim
```

## Build Systems


| Build System| Supported |
| ------------- | ------------- |
| CMake | :heavy_check_mark: |
| Premake | :heavy_multiplication_x: &nbsp; :soon: &nbsp; :heavy_check_mark: |
| Make | :heavy_multiplication_x: &nbsp; :soon: &nbsp; :heavy_check_mark: |
