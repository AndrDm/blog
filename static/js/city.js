

function verboseDraw(ctx, t) {
  const w = 99;
  ctx.clearRect(0, 0, 100, 100);
  // Instead of two loops for horizontal and vertical we just do one scanning both.
  for(let i=6e3;i--;) {
    // World space coordinates.
    let X=t;
    let Y=1;
    let Z;
    // Ray direction in world space.
    let a=i%w/50-1;
    let b=1-i/4e3;
    let d=1; // Light.
    // Background color.
    let s=
      BACKGROUND==='Original'? b:
      BACKGROUND==='Beam'? a*b:
      BACKGROUND==='Sunny'? Math.max(0, Math.hypot(a-0.6,b-0.4)-0.2):
      0;
    // Ray tracing!
    for (Z=2; Z<w; ++Z) {
      if (isThereAnythingAt(X, Y, Z, t)) {
        // We hit the model. We will continue ray tracing, but now toward the light.
        const last_d = d;
        // The texture.
        s=TEXTURE==='Original'? (X&Y&Z)%3:
          TEXTURE==='Striped'? Y&4:
          TEXTURE==='None'? 0:
          1;
        s/=Z; // Fog.
        a=b=1; // New direction, toward the light.
        d=Z/w;
        if (last_d < 1) break; // Effectively the same as the ||d|(...) part.
        // Checking the previous "d" instead of the current one lets the light penetrate one
        // voxel deep, creating rim lights. (Super nice on the top of the buildings.)
      }
      X+=a;
      Y-=b;
    }
    const darkness = s+1-d*Z/w; // Texture + lighting.
    // Each pixel is colored by drawing a rectangle on it that is 0-1 pixel wide.
    ctx.fillRect(i%w, i/w|0, darkness, 1);
  }
}


function isThereAnythingAt(X, Y, Z, t) {
  if (MODEL === 'Original') {
    // The random building height comes from (X^Z)*8.
    return Y>=GROUND_PLANE-(CITY_DISTANCE<Z&AVENUE_WIDTH<X%AVENUE_PERIOD&&X/BUILDING_WIDTH^Z/BUILDING_DEPTH)*8%(BUILDING_HEIGHT+1);
  } else if (MODEL === 'Simple cubes') {
    return Z%50>30 && (Y+110)%50<20 && X%50<20;
  } else if (MODEL === 'Torus') {
    X -= t;
    Z -= 70;
    Y += 10;
    function rot(x, y, theta) {
      const s = Math.sin(theta);
      const c = Math.cos(theta);
      return [c*x+s*y, c*y-s*x];
    }
    [X, Y] = rot(X, Y, t/50);
    [Y, Z] = rot(Y, Z, t/20);
    const q = Math.hypot(X, Z)-20;
    return Math.hypot(q, Y)-10 < 0;
  }
}

function originalDraw(ctx, t) {
  ctx.clearRect(0, 0, 100, 100);
  let i, X, Y, Z, a, b, d, s, w;
  // This is from the tweet almost verbatim.
  for(w=99,i=6e3;i--;ctx.fillRect(i%w,i/w|0,1-d*Z/w+s,1))
  for(a=i%w/50-1,s=b=1-i/4e3,X=t,Y=Z=d=1;
      ++Z<w&(Y<6-(32<Z&27<X%w&&X/9^Z/8)*8%46||d|(s=(X&Y&Z)%3/Z,a=b=1,d=Z/w));
      Y-=b)X+=a;
}

