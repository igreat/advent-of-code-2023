from z3 import Solver, Real, simplify, sat

def part1():
    solver = Solver()

    x, y, z, v_x, v_y, v_z = Real('x'), Real('y'), Real('z'), Real('v_x'), Real('v_y'), Real('v_z')

    # define the sets of points and velocities
    points = [
        (219051609191782, 68260434807407, 317809635461867, 146, 364, -22),
        (292151991892724, 394725036264709, 272229701860796, -43, -280, -32),
        (455400538938496, 167482380286201, 389150487664328, -109, 219, -58),
    ]

    for i in range(len(points)):
        # add t variable for each point
        t = Real(f't{i}')
        points[i] = points[i] + (t,)

    # add constraints
    for i, (x_i, y_i, z_i, v_i_x, v_i_y, v_i_z, t) in enumerate(points):
        solver.add(x + v_x * t == x_i + v_i_x * t)
        solver.add(y + v_y * t == y_i + v_i_y * t)
        solver.add(z + v_z * t == z_i + v_i_z * t)

    if solver.check() == sat:
        model = solver.model()
        solution = {str(var): model[var] for var in [x, y, z, v_x, v_y, v_z, t]}
        for var in solution:
            print(f"{var} = {solution[var]}")
        # sum up x + y + z
        sum_xyz = simplify(model[x] + model[y] + model[z])
        print(f"Sum of x, y, and z: {sum_xyz}")
    else:
        print("No solution found")

if __name__ == '__main__':
    # why parse the input when you can just hardcode it :P
    part1()