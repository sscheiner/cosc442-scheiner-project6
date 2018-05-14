<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Location 1</name>
   <tag></tag>
   <elementGuidId>38dc607e-1541-46c7-b693-e829fa97411a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>panel-body</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>

            
            Location 1
            

                 Washington, United States   
              


            
            
            
            
            $(function(){

                    $(&quot;#locationlist1&quot;).select2(
                      {
                              placeholder: &quot;Enter Location&quot;,
                              minimumInputLength: 3,
                              width:'100%', maximumSelectionSize: 1,
                              initSelection: function (element, callback) {
                                      var data = {id: &quot;1&quot;, text: &quot;&quot;};
                                      callback(data);
                                  },
                              ajax: {
                                  url: &quot;https://www.phptravels.net/admin/ajaxcalls/locationsList&quot;,
                                  dataType: 'json',
                                  data: function (term, page) {
                                      return {
                                          query: term, // search term

                                      };
                                  },
                                  results: function (data, page) {

                                      return {results: data};
                                  }
                              }
                          }
                     );

                      $(&quot;#locationlist1&quot;).on(&quot;select2-selecting&quot;, function(e) {
                       $(&quot;#locationid1&quot;).val(e.val);
                       console.log(e.val);
                    });
            })
            
             
            Location 2
            

                 Enter Location                      
              


            
            
            
            
            $(function(){

                    $(&quot;#locationlist2&quot;).select2(
                      {
                              placeholder: &quot;Enter Location&quot;,
                              minimumInputLength: 3,
                              width:'100%', maximumSelectionSize: 1,
                              initSelection: function (element, callback) {
                                      var data = {id: &quot;1&quot;, text: &quot;&quot;};
                                      callback(data);
                                  },
                              ajax: {
                                  url: &quot;https://www.phptravels.net/admin/ajaxcalls/locationsList&quot;,
                                  dataType: 'json',
                                  data: function (term, page) {
                                      return {
                                          query: term, // search term

                                      };
                                  },
                                  results: function (data, page) {

                                      return {results: data};
                                  }
                              }
                          }
                     );

                      $(&quot;#locationlist2&quot;).on(&quot;select2-selecting&quot;, function(e) {
                       $(&quot;#locationid2&quot;).val(e.val);
                       console.log(e.val);
                    });
            })
            
             
            Location 3
            

                 Enter Location                      
              


            
            
            
            
            $(function(){

                    $(&quot;#locationlist3&quot;).select2(
                      {
                              placeholder: &quot;Enter Location&quot;,
                              minimumInputLength: 3,
                              width:'100%', maximumSelectionSize: 1,
                              initSelection: function (element, callback) {
                                      var data = {id: &quot;1&quot;, text: &quot;&quot;};
                                      callback(data);
                                  },
                              ajax: {
                                  url: &quot;https://www.phptravels.net/admin/ajaxcalls/locationsList&quot;,
                                  dataType: 'json',
                                  data: function (term, page) {
                                      return {
                                          query: term, // search term

                                      };
                                  },
                                  results: function (data, page) {

                                      return {results: data};
                                  }
                              }
                          }
                     );

                      $(&quot;#locationlist3&quot;).on(&quot;select2-selecting&quot;, function(e) {
                       $(&quot;#locationid3&quot;).val(e.val);
                       console.log(e.val);
                    });
            })
            
             
            Location 4
            

                 Enter Location                      
              


            
            
            
            
            $(function(){

                    $(&quot;#locationlist4&quot;).select2(
                      {
                              placeholder: &quot;Enter Location&quot;,
                              minimumInputLength: 3,
                              width:'100%', maximumSelectionSize: 1,
                              initSelection: function (element, callback) {
                                      var data = {id: &quot;1&quot;, text: &quot;&quot;};
                                      callback(data);
                                  },
                              ajax: {
                                  url: &quot;https://www.phptravels.net/admin/ajaxcalls/locationsList&quot;,
                                  dataType: 'json',
                                  data: function (term, page) {
                                      return {
                                          query: term, // search term

                                      };
                                  },
                                  results: function (data, page) {

                                      return {results: data};
                                  }
                              }
                          }
                     );

                      $(&quot;#locationlist4&quot;).on(&quot;select2-selecting&quot;, function(e) {
                       $(&quot;#locationid4&quot;).val(e.val);
                       console.log(e.val);
                    });
            })
            
             
            Location 5
            

                 Enter Location                      
              


            
            
            
            
            $(function(){

                    $(&quot;#locationlist5&quot;).select2(
                      {
                              placeholder: &quot;Enter Location&quot;,
                              minimumInputLength: 3,
                              width:'100%', maximumSelectionSize: 1,
                              initSelection: function (element, callback) {
                                      var data = {id: &quot;1&quot;, text: &quot;&quot;};
                                      callback(data);
                                  },
                              ajax: {
                                  url: &quot;https://www.phptravels.net/admin/ajaxcalls/locationsList&quot;,
                                  dataType: 'json',
                                  data: function (term, page) {
                                      return {
                                          query: term, // search term

                                      };
                                  },
                                  results: function (data, page) {

                                      return {results: data};
                                  }
                              }
                          }
                     );

                      $(&quot;#locationlist5&quot;).on(&quot;select2-selecting&quot;, function(e) {
                       $(&quot;#locationid5&quot;).val(e.val);
                       console.log(e.val);
                    });
            })
            
             
            Location 6
            

                 Enter Location                      
              


            
            
            
            
            $(function(){

                    $(&quot;#locationlist6&quot;).select2(
                      {
                              placeholder: &quot;Enter Location&quot;,
                              minimumInputLength: 3,
                              width:'100%', maximumSelectionSize: 1,
                              initSelection: function (element, callback) {
                                      var data = {id: &quot;1&quot;, text: &quot;&quot;};
                                      callback(data);
                                  },
                              ajax: {
                                  url: &quot;https://www.phptravels.net/admin/ajaxcalls/locationsList&quot;,
                                  dataType: 'json',
                                  data: function (term, page) {
                                      return {
                                          query: term, // search term

                                      };
                                  },
                                  results: function (data, page) {

                                      return {results: data};
                                  }
                              }
                          }
                     );

                      $(&quot;#locationlist6&quot;).on(&quot;select2-selecting&quot;, function(e) {
                       $(&quot;#locationid6&quot;).val(e.val);
                       console.log(e.val);
                    });
            })
            
             
            Location 7
            

                 Enter Location                      
              


            
            
            
            
            $(function(){

                    $(&quot;#locationlist7&quot;).select2(
                      {
                              placeholder: &quot;Enter Location&quot;,
                              minimumInputLength: 3,
                              width:'100%', maximumSelectionSize: 1,
                              initSelection: function (element, callback) {
                                      var data = {id: &quot;1&quot;, text: &quot;&quot;};
                                      callback(data);
                                  },
                              ajax: {
                                  url: &quot;https://www.phptravels.net/admin/ajaxcalls/locationsList&quot;,
                                  dataType: 'json',
                                  data: function (term, page) {
                                      return {
                                          query: term, // search term

                                      };
                                  },
                                  results: function (data, page) {

                                      return {results: data};
                                  }
                              }
                          }
                     );

                      $(&quot;#locationlist7&quot;).on(&quot;select2-selecting&quot;, function(e) {
                       $(&quot;#locationid7&quot;).val(e.val);
                       console.log(e.val);
                    });
            })
            
             
            Location 8
            

                 Enter Location                      
              


            
            
            
            
            $(function(){

                    $(&quot;#locationlist8&quot;).select2(
                      {
                              placeholder: &quot;Enter Location&quot;,
                              minimumInputLength: 3,
                              width:'100%', maximumSelectionSize: 1,
                              initSelection: function (element, callback) {
                                      var data = {id: &quot;1&quot;, text: &quot;&quot;};
                                      callback(data);
                                  },
                              ajax: {
                                  url: &quot;https://www.phptravels.net/admin/ajaxcalls/locationsList&quot;,
                                  dataType: 'json',
                                  data: function (term, page) {
                                      return {
                                          query: term, // search term

                                      };
                                  },
                                  results: function (data, page) {

                                      return {results: data};
                                  }
                              }
                          }
                     );

                      $(&quot;#locationlist8&quot;).on(&quot;select2-selecting&quot;, function(e) {
                       $(&quot;#locationid8&quot;).val(e.val);
                       console.log(e.val);
                    });
            })
            
             
            Location 9
            

                 Enter Location                      
              


            
            
            
            
            $(function(){

                    $(&quot;#locationlist9&quot;).select2(
                      {
                              placeholder: &quot;Enter Location&quot;,
                              minimumInputLength: 3,
                              width:'100%', maximumSelectionSize: 1,
                              initSelection: function (element, callback) {
                                      var data = {id: &quot;1&quot;, text: &quot;&quot;};
                                      callback(data);
                                  },
                              ajax: {
                                  url: &quot;https://www.phptravels.net/admin/ajaxcalls/locationsList&quot;,
                                  dataType: 'json',
                                  data: function (term, page) {
                                      return {
                                          query: term, // search term

                                      };
                                  },
                                  results: function (data, page) {

                                      return {results: data};
                                  }
                              }
                          }
                     );

                      $(&quot;#locationlist9&quot;).on(&quot;select2-selecting&quot;, function(e) {
                       $(&quot;#locationid9&quot;).val(e.val);
                       console.log(e.val);
                    });
            })
            
             
            Location 10
            

                 Enter Location                      
              


            
            
            
            
            $(function(){

                    $(&quot;#locationlist10&quot;).select2(
                      {
                              placeholder: &quot;Enter Location&quot;,
                              minimumInputLength: 3,
                              width:'100%', maximumSelectionSize: 1,
                              initSelection: function (element, callback) {
                                      var data = {id: &quot;1&quot;, text: &quot;&quot;};
                                      callback(data);
                                  },
                              ajax: {
                                  url: &quot;https://www.phptravels.net/admin/ajaxcalls/locationsList&quot;,
                                  dataType: 'json',
                                  data: function (term, page) {
                                      return {
                                          query: term, // search term

                                      };
                                  },
                                  results: function (data, page) {

                                      return {results: data};
                                  }
                              }
                          }
                     );

                      $(&quot;#locationlist10&quot;).on(&quot;select2-selecting&quot;, function(e) {
                       $(&quot;#locationid10&quot;).val(e.val);
                       console.log(e.val);
                    });
            })
            
             
            </value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;GENERAL&quot;)/div[@class=&quot;panel panel-default&quot;]/div[@class=&quot;panel-body&quot;]</value>
   </webElementProperties>
</WebElementEntity>
