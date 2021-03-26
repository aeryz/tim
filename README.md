
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

2. Write some tim-tests in files that end with `_test.c`.
```c
TIM_TEST(cool_stuff) {
    TIM_ASSERT_EQ(true, false);
    TIM_SUCCESS();
}
```
3. Create a `CMakeLists.txt` in the project root.

4. Write some cmake stuff.
```cmake
if (DEFINED TIM_TEST_BUILD)
    add_library(my_awesome_lib SHARED foo_test.c bar_test.c)
endif()
```

5. Run TIM.
```bash
tim -p /path/to/my/project
```

## Build Systems 


| Build System| Supported |
| ------------- | ------------- |
| CMake | :heavy_multiplication_x: &nbsp; :soon: &nbsp; :heavy_check_mark: |
| Premake | :heavy_multiplication_x: &nbsp; :soon: &nbsp; :heavy_check_mark: |
| Make | :heavy_multiplication_x: &nbsp; :soon: &nbsp; :heavy_check_mark: |