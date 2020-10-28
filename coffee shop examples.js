// https://enkimute.github.io/ganja.js/examples/coffeeshop.html#cga2d_points_and_circles
// Create a Clifford Algebra with 3,1 metric for 2D CGA. 
Algebra(3,1,()=>{ 

// The conformal model adds in more element types. (circles, point-pairs)
// We no longer work in a dual space. (so ^ = join and & = meet)
// Vectors are points, Bivectors are point pairs, Trivectors are lines/circles

// We don't work directly in the e3/e4 basis, but instead rotate it so we have
// two null vectors to work with (called origin and infinite) 
 var ni = 1e4+1e3,           // n-infinite
      no = .5e4-.5e3;         // n-origin
  
// Define points, lines, circles using the null basis.  
  var point  = (x,y)=>no + x*1e1 + y*1e2 + 0.5*(x*x+y*y)*ni,
      line   = (a,b,c)=>!(a*1e1 + b*1e2 + c*ni),
      circle = (x,y,r)=>!(point(x,y) - r**2/2*ni);
  
// Distances and Angles. 
  var dist=(x,y)=>(2*(x<<y).Length)**0.5,
      angle=(x,y)=>Math.acos(!x.Normalized<<!y.Normalized);

// Define three points
  var p1 = point(-0.5, -0.5),
      p2 = point( 1, -0.5), 
      p3 = point(   0,  1.5);
  
// Define two circles, one by wedging 3 points, one directly.
  var C = ()=>p1^p2^p3,        // a function so it updates live.
      D = circle(1,-1,0.9);
  
// Define two lines, one directly, one by wedging two points and infinity.
  var X=line(0,1,0), Y=()=>p2^p3^ni;
      
// Create point pairs by intersecting circle(s) and a line(s).
  var pp1=()=>X&C, pp2=()=>C&D, pp3=()=>Y&D, p4=()=>no|(X&Y);
      
// Graph these items.
  document.body.appendChild(this.graph([
    "2D CGA - drag p1,p2,p3","",                  // title
    0xFF8888, C, "C", D, "D",                     // circles
    0x44AA44, X, "X", Y, "Y", p4,                 // lines
    0x4444FF, pp1, "pp1", pp2, "pp2", pp3, "pp3", // point pairs
    0x666666, p1, "p1", p2, "p2", p3, "p3",       // points
  ],{conformal:true,grid:true}));                 // conformal flag!  

});






































// https://enkimute.github.io/ganja.js/examples/coffeeshop.html#cga2d_project_and_reject
// Create a Clifford Algebra with 3,1 metric for 2D CGA. 
Algebra(3,1,()=>{ 

// Two null vectors to work with (called origin and infinite)
  var ni = 1e4+1e3,           // n-infinity
      no = .5e4-.5e3;         // n-origin
  
// Define points, lines, circles using the null basis.  
  var point  = (x,y)=>no + x*1e1 + y*1e2 + 0.5*(x*x+y*y)*ni,
      line   = (a,b,c)=>!(a*1e1 + b*1e2 + c*ni),
      circle = (x,y,r)=>!(point(x,y) - r**2/2*ni);
  
// Some identities
  var line_through_P_parallel_with_X = (P,X)=>(P^ni)<<X*(P^ni),
      project_point_on_circle        = (P,C)=>(P^ni)<<C*C,
      project_point_on_line          = (P,L)=>P<<L.Normalized*L.Normalized,
      position                       = (X)=>{ X=X/(X^ni); return X/(X<<-ni); },
      radius                         = (X)=>Math.abs(((X<<X).s/((X^ni)**2).s))**.5,
      attitude                       = (X)=>(ni^no<<(X^ni)).Normalized,
      split                          = (pp)=>position(pp)-radius(pp)*attitude(pp);

// Define some elements.
  var p1 = point(-0.7,0.4), p2 = point(1,-0.5), p3 = point(0,1.5),
      D = circle(1,-1,0.7),
      X = line(0,1,0), Y=()=>p2^p3^ni;
      
// Graph 
  document.body.appendChild(this.graph([
    "2D CGA - drag p1,p2,p3","",                        // title
    0xFF8888, D,                                        // circle
    ()=>position(D), ()=>"D "+(radius(D).toFixed(2)),   // Center,Radius
    0x44AA44, X, "X", Y, "Y",                           // lines
    0x4444FF,
    ()=>line_through_P_parallel_with_X(p1,Y),"p1 // Y", // parallel to line
    ()=>line_through_P_parallel_with_X(p1,D),"p1 // D", // parallel to circle
    0xFF44FF, 
    ()=>split(project_point_on_circle(p1,D)),"p1 on D", // project on circle
    ()=>project_point_on_line(p1,X), "p1 on X",         // project on line
    ()=>project_point_on_line(p1,Y), "p1 on Y",         // project on line
    0x444444,p1,"p1",p2,"p2",p3,"p3",                   // render the points
  ],{conformal:true,grid:true}));                       // conformal flag!  

});


































// https://enkimute.github.io/ganja.js/examples/coffeeshop.html#cga2d_rotors_and_translators

// Create a Clifford Algebra with 3,1 metric for 2D CGA. 
Algebra(3,1,()=>{ 

// We don't work directly in the e3/e4 basis, but instead rotate it so we have
// two null vectors to work with (called origin and infinite)
  var ni = 1e4+1e3, no = 0.5*(1e4-1e3),
      point = (x,y)=>no + x*1e1 + y*1e2 + 0.5*(x*x+y*y)*ni;
  
// Rotator, Translator
  var translate = (v)=>(1-.5*v^ni),
      rotate    = (P,a)=>Math.cos(a/2) - Math.sin(a/2)*(1e12-P<<1e12^ni);
 
// Define a translation and a rotation and apply them to two points
  var tr = ()=>translate( Math.sin(performance.now()/1000)*1e1 ),
      ro = ()=>rotate(p1,performance.now()/1000)*tr,
      p1 = ()=>tr>>>point( 0, 0 ),
      p2 = ()=>ro>>>(point( 0.75, 0 ));
      
// Graph these items.
  document.body.appendChild(this.graph([
      p1,"tr>>>p1",                       // point 1
      p2,"rt>>>p2",                       // point 2
      ()=>[p1,p2],                        // segment from 1 to 2
      ()=>p1^p2^point(0,2,0)              // pretty circle
  ],{conformal:true,animate:true,grid:true}));
  
});





































// https://enkimute.github.io/ganja.js/examples/coffeeshop.html#cga2d_euler_line
// Create a Clifford Algebra with 3,1 metric for 2D CGA.
Algebra(3,1,()=>{

  // The Euler line connects several important points in any non equilateral triangle.
  // We repeat the example from 2D PGA using our new conformal tools.

  // Null basis and point helper.
  var ni = 1e4+1e3, no = 0.5*(1e4-1e3), 
      point  = (x,y)=>no + x*1e1 - y*1e2 + 0.5*(x*x+y*y)*ni,

  // Three points.
      A=point(-1.5,0.5), B=point(0.5,-1.5), C=point(1,0.5),
  
  // Circle around triangle.
      circle = ()=>A^B^C,     
      circum = ()=>-circle/(ni<<circle),
      
  // The centroid      
      centroid = ()=>(A+B+C)/3,
  
  // The Euler line      
      line = ()=>centroid^ni^circum;
   
  document.body.appendChild(this.graph([
      0xFFFFEE,[A,B,C],                        // triangle
      0x88AA88,circum,circle,                  // circumcenter
      0x8888FF,centroid,line,"Euler Line",     // centroid and euler line
      0x444444,A,B,C,                          // triangle vertices
  ],{conformal:true, grid:true}));

});





























// https://enkimute.github.io/ganja.js/examples/coffeeshop.html#cga2d_circle_fit
// https://www.researchgate.net/publication/266149530
// Circle LLS fitting. MATRIX FREE version by enki.

// Find a single eigenvector/value (orthogonal to the list in orth)
var GA_Rayleigh = Algebra(4,0,1).inline(function(P,mu=0,orth=[],g=(1e1+1e2+1e3+1e4).Normalized) {
      P = P.map(x=>[...x.Vector]*[1e1,1e2,1e3,-1e4])                                         // note -1e4 from our source algebra ..  
      orth=orth.map(o=>{o=o.vec*[1e1,1e2,1e3,-1e4]; g=(g-(g|o)*o).Normalized; return o;})    // orthogonalize guess
      for (var i=0; i<30; i++) {
        var sol = (!P.reduce((s,P,j)=>s=s^(P-g[2+j]*1e0-Element.Coeff(mu,j)),1)).Normalized  // solve with OP
        orth.forEach(orth=>sol=(sol-(sol|orth)*orth).Normalized);                            // orthogonalize
        var newmu = mu+sol.e0*[sol.e1,sol.e2,sol.e3,sol.e4]*[g.e1,g.e2,g.e3,g.e4];           // update eigenvalue
        g = sol; if (Math.abs(newmu-mu)<1E-8) break; mu=newmu                                // stop when done
      }
      return {vec:[...g.Vector.slice(1)],val:mu.s};
    });

// Find all eigenvectors given a vector of covariant planes P, and sort them small->big
var GA_Eigen = P=>P.reduce((s,p)=>s.concat([GA_Rayleigh(P,0,s)]),[]).sort((a,b)=>a.val-b.val) 

// Now create a 2D conformal geometric algebra.
Algebra(3,1,()=>{
    // Standard conformal setup.
    var no=.5e4-.5e3, ni=1e4+1e3, pss=1e1234,     // null basis and pss
        up=x=>no+x+0.5*x*x*ni;                    // conformal vec->point
    
    // Generate noisy points on circle    
    var pts=[...Array(6)].map(x=>{
       var alpha  = Math.random()*2*Math.PI, radius = 1 + Math.random()*0.05-0.025;
       return up(Math.sin(alpha)*radius*1e1 + Math.cos(alpha)*radius*1e2);
    });
    
    // Compute covariance planes, find eigen circles and render ..
    document.body.appendChild(this.graph(()=>{
    // Calculate vector of covariance planes     
      var planes=[1,2,3,4].map(j=>pts.reduce((s,P)=>s+P*P[j],0))
    // Find two smallest positive eigenvectors and convert them to circles  
      var eigen = GA_Eigen(planes).filter(x=>x.val>0).map(x=>!(x.vec*[1e1,1e2,1e3,1e4]));
    // Render the points, circles and intersection of circles (best point pair)  
      return  [ 0xff0000,"drag the points..",0x0000ff,"Best circle",0xff00ff,"Best point pair",
                0xff0000,...pts,0x0000ff,eigen[0], 0xffaaff,eigen[1], 0xff00ff,eigen[0]&eigen[1],
    ]},{conformal:true,animate:1,pointRadius:1,grid:1}));
})