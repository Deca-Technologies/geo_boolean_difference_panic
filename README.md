# geo_boolean_difference_panic

[![Run main](https://github.com/Deca-Technologies/geo_boolean_difference_panic/actions/workflows/run.yml/badge.svg)](https://github.com/Deca-Technologies/geo_boolean_difference_panic/actions/workflows/run.yml)

Demonstration of a panic in the geo::BooleanOps::difference function.

## Problem

About half of the time, the BooleanOps::difference function fails to difference the two multipolygons included. Attempts to isolate by generating a simpler case failed. The test data was isolated from a larger test case.

## Debugging notes
                                                                                                
- Test case included is one large polygon containing many small polygons and is reduced from a larger case
- Panic does not occur on every call and only happens about 50% of the time
- Adding more polygons to the multipolygon will sometimes cause a success
- Only occurs on large multipolygons diff'ed by a multipolygon made up of a single large polygon
