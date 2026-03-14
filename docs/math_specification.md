# Mathematical Specification: ASCII Torus Renderer

This document gives the complete mathematical derivation behind the ASCII
torus renderer implemented in `src/lib.rs`.


## 1. Torus Parametric Equations

A torus centred at the origin with its symmetry axis along Y is defined by
two parameters:

    theta (t) in [0, 2*pi)   -- angle around the tube (minor circle)
    phi   (p) in [0, 2*pi)   -- angle around the ring (major circle)

and two radii:

    R2 = 2   (major radius -- centre of tube to centre of torus)
    R1 = 1   (minor radius -- radius of the tube itself)

A point on the unrotated torus surface is:

    [ x ]   [ (R2 + R1*cos(t)) * cos(p) ]
    [ y ] = [  R1 * sin(t)               ]
    [ z ]   [ (R2 + R1*cos(t)) * sin(p)  ]

Define the shorthand:

    circle_x = R2 + R1*cos(t)
    circle_y = R1*sin(t)


## 2. Rotation Matrices

The torus is animated by two time-varying angles:

    A  -- rotation about the X-axis  (incremented by dA = 0.07 per frame)
    B  -- rotation about the Z-axis  (incremented by dB = 0.03 per frame)

### Rx(A) -- rotation about X

    Rx(A) = [ 1     0       0    ]
            [ 0   cos(A)  -sin(A)]
            [ 0   sin(A)   cos(A)]

### Rz(B) -- rotation about Z

    Rz(B) = [ cos(B)  -sin(B)  0 ]
            [ sin(B)   cos(B)  0 ]
            [   0        0     1 ]

### Composed rotation: Rz(B) * Rx(A)

The full rotation matrix applied to every point is M = Rz(B) * Rx(A):

    M = [ cos(B)   -sin(B)*cos(A)   sin(B)*sin(A) ]
        [ sin(B)    cos(B)*cos(A)  -cos(B)*sin(A)  ]
        [   0         sin(A)          cos(A)        ]


## 3. Rotated Point Coordinates

Applying M to the unrotated point [circle_x*cos(p), circle_y, circle_x*sin(p)]:

    x' = circle_x * (cos(B)*cos(p) + sin(A)*sin(B)*sin(p))
       - circle_y * cos(A)*sin(B)

    y' = circle_x * (sin(B)*cos(p) - sin(A)*cos(B)*sin(p))
       + circle_y * cos(A)*cos(B)

    z' = K2 + cos(A)*circle_x*sin(p) + circle_y*sin(A)

The viewer sits at z=0 looking in the +z direction, so the constant K2 = 5
is added to z to push the torus in front of the camera. Every visible point
has z' > 0.


## 4. Perspective Projection

### Projection formulae

Screen coordinates (integer pixel positions) are obtained by perspective
division:

    ooz = 1 / z'                           (reciprocal depth)

    xp = floor( W/2  +  K1 * ooz * x' )
    yp = floor( H/2  -  K1 * ooz * y' / 2 )

where:

    W = 80   (screen width  in columns)
    H = 24   (screen height in rows)

### K1 derivation

K1 controls the projected size of the torus on screen. It is chosen so
that the torus fills a reasonable fraction of the screen width:

    K1 = W * K2 / (8 * (R1 + R2))
       = 80 * 5 / (8 * 3)
       = 400 / 24
       ~ 16.67

(The factor of 8 in the denominator is a design choice to leave horizontal
margin.)

### Aspect-ratio correction

Terminal characters are roughly twice as tall as they are wide. The factor
of 2 in the yp denominator compensates for this:

    yp uses  K1 * ooz * y' / 2

so that a circle in 3-D projects to a circle on screen rather than a
vertically-stretched ellipse.


## 5. Surface Normal and Luminance

### Surface normal (parametric derivation)

For a parametric surface P(t,p), the outward unit normal is proportional to
dP/dt x dP/dp. For the unrotated torus the outward unit normal simplifies
to:

    n_local = [ cos(t)*cos(p),  sin(t),  cos(t)*sin(p) ]

(This is already unit length because R1 cancels in the cross product.)

The rotated normal is n = M * n_local, giving:

    nx = cos(t)*(cos(B)*cos(p) + sin(A)*sin(B)*sin(p)) - cos(A)*sin(B)*sin(t)
    ny = cos(t)*(sin(B)*cos(p) - sin(A)*cos(B)*sin(p)) + cos(A)*cos(B)*sin(t)
    nz = cos(A)*cos(t)*sin(p) + sin(A)*sin(t)

### Light direction

The (unnormalised) light vector is:

    L = (0, 1, -1)

pointing upward and toward the viewer.

### Luminance computation

Luminance is the dot product of the rotated normal with L:

    luminance = dot(n, L)
              = ny - nz

Expanding:

    luminance = cos(p)*cos(t)*sin(B)
              - cos(A)*cos(t)*sin(p)
              - sin(A)*sin(t)
              + cos(B)*(cos(A)*sin(t) - cos(t)*sin(A)*sin(p))

Only points with luminance > 0 face the light and are rendered (back-face
culling). The luminance value is mapped to an ASCII character from a
12-level palette:

    ".,-~:;=!*#$@"

via:

    index = clamp( floor(luminance * 8), 0, 11 )
    char  = palette[index]

The light vector is NOT normalised (|L| = sqrt(2)), which means the raw
dot product can exceed 1.0. Multiplying by 8 and clamping to [0,11]
absorbs this and spreads the range across all 12 characters.


## 6. Z-Buffer

A per-pixel depth buffer stores the reciprocal depth (1/z') of the closest
point projected to each screen cell. For each sample:

    1. Compute ooz = 1/z'.
    2. Project to screen pixel (xp, yp).
    3. If luminance > 0 and (xp, yp) is within screen bounds:
       a. If ooz > zbuffer[yp][xp], this point is closer:
          - Update zbuffer[yp][xp] = ooz
          - Update output[yp][xp]  = luminance_to_char(luminance)

Using 1/z rather than z means "closer = larger value", so the comparison
is a simple `>`.  The buffer is initialised to 0.0 each frame, which
represents infinitely far away (1/z -> 0 as z -> infinity).


## 7. Sampling Strategy

The torus surface is sampled on a uniform grid in (theta, phi) space:

    theta_step = 0.07   =>  ~89 steps  (2*pi / 0.07 ~ 89.8)
    phi_step   = 0.02   =>  ~314 steps (2*pi / 0.02 ~ 314.2)

    Total points per frame ~ 89 * 314 ~ 27,946  (approximately 28k)

The theta step is coarser than phi because the tube (minor circle) is
smaller than the ring (major circle), so fewer samples are needed for
comparable visual density.


## 8. Complete Expanded Per-Point Formula

For each sample point with precomputed trig values sin(t), cos(t), sin(p),
cos(p), sin(A), cos(A), sin(B), cos(B):

    Step 1 -- helper values:
        circle_x = R2 + R1*cos(t)          = 2 + cos(t)
        circle_y = R1*sin(t)               = sin(t)

    Step 2 -- rotated 3-D position:
        x = circle_x*(cos(B)*cos(p) + sin(A)*sin(B)*sin(p))
          - circle_y*cos(A)*sin(B)

        y = circle_x*(sin(B)*cos(p) - sin(A)*cos(B)*sin(p))
          + circle_y*cos(A)*cos(B)

        z = K2 + cos(A)*circle_x*sin(p) + circle_y*sin(A)

    Step 3 -- reciprocal depth:
        ooz = 1 / z

    Step 4 -- screen projection:
        xp = floor( 40 + K1 * ooz * x )
        yp = floor( 12 - K1 * ooz * y / 2 )

    Step 5 -- luminance:
        L  = cos(p)*cos(t)*sin(B)
           - cos(A)*cos(t)*sin(p)
           - sin(A)*sin(t)
           + cos(B)*(cos(A)*sin(t) - cos(t)*sin(A)*sin(p))

    Step 6 -- z-buffer test & character write:
        if L > 0 and 0 <= xp < 80 and 0 <= yp < 24:
            if ooz > zbuffer[yp][xp]:
                zbuffer[yp][xp] = ooz
                output[yp][xp]  = ".,-~:;=!*#$@"[clamp(floor(L*8), 0, 11)]


## 9. Summary Parameter Table

    Parameter      Symbol   Value     Description
    ─────────────  ──────   ────────  ──────────────────────────────────
    Major radius   R2       2         Centre of tube to centre of torus
    Minor radius   R1       1         Radius of the tube cross-section
    Viewer dist.   K2       5         Camera-to-torus-centre distance
    Scale factor   K1       ~16.67    W*K2 / (8*(R1+R2))
    Screen width   W        80        Terminal columns
    Screen height  H        24        Terminal rows
    Theta step     d_theta  0.07      Sampling step around the tube
    Phi step       d_phi    0.02      Sampling step around the ring
    X-rot incr.    dA       0.07      Rotation-A increment per frame
    Z-rot incr.    dB       0.03      Rotation-B increment per frame
    Frame delay    --       33 ms     Approximate target: ~30 fps
    Luminance map  --       12 chars  ".,-~:;=!*#$@"
    Light dir.     L        (0,1,-1)  Unnormalised; upward and toward viewer
    Points/frame   --       ~28,000   89 theta * 314 phi samples


## 10. Frame Rendering Algorithm (Pseudocode)

    INITIALISE A = 0, B = 0
    K1 = W * K2 / (8 * (R1 + R2))

    LOOP forever:
        -- Clear buffers
        output[0..H][0..W]  = ' '
        zbuffer[0..H][0..W] = 0.0

        -- Precompute frame-level trig
        sin_A = sin(A);  cos_A = cos(A)
        sin_B = sin(B);  cos_B = cos(B)

        FOR theta = 0, 0.07, 0.14, ... < 2*pi:
            sin_t = sin(theta);  cos_t = cos(theta)

            FOR phi = 0, 0.02, 0.04, ... < 2*pi:
                sin_p = sin(phi);  cos_p = cos(phi)

                -- Compute rotated position, ooz, projection, luminance
                --   (full formulae in Section 8)

                IF luminance > 0 AND pixel in bounds AND ooz > zbuffer[yp][xp]:
                    zbuffer[yp][xp] = ooz
                    output[yp][xp]  = palette[clamp(floor(luminance*8), 0, 11)]

        -- Emit frame
        MOVE cursor to home position (ANSI: ESC[H)
        FOR each row in output:
            PRINT row as string + newline
        FLUSH stdout

        -- Advance rotation
        A = A + 0.07
        B = B + 0.03

        SLEEP 33 ms
