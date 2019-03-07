<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Create project with template id</description>
   <name>Create project with template id</name>
   <tag></tag>
   <elementGuidId>6e2d3ab9-9003-44dd-9f44-7e78a1dddc79</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\&quot;autoSelectTemplateFlag\&quot;: false,\n\&quot;category\&quot;: \&quot;Project\&quot;,\n\&quot;organizationId\&quot;: \&quot;${OrgId}\&quot;,\n\&quot;shortName\&quot;: \&quot;Project with template id\&quot;,\n\&quot;taskTemplateId\&quot;: 275,\n\&quot;tzOffsetPlanned\&quot;: 330,\n\&quot;projectFieldMap\&quot;: {\n\&quot;25034\&quot;: \&quot;Test\&quot;,\n\&quot;25036\&quot;: \&quot;1\&quot;,\n\&quot;25038\&quot;: \&quot;a\&quot;,\n\&quot;25040\&quot;: \&quot;3218\&quot;,\n\&quot;25041\&quot;: \&quot;12/31/2018\&quot;,\n\&quot;25042\&quot;: \&quot;Yes\&quot;,\n\&quot;25056\&quot;: \&quot;Project\&quot;,\n\&quot;26376\&quot;: \&quot;1\&quot;\n}\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${Authorization}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}/rest/projects?</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>findTestData('Create Entity').getValue(1, 1)</defaultValue>
      <description></description>
      <id>083ae745-0e6e-43cd-8da6-4ea3a095dd82</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>findTestData('Create Entity').getValue(2, 1)</defaultValue>
      <description></description>
      <id>12a7ae0e-0a12-49f5-9d5f-b8f7bf2aecf8</id>
      <masked>false</masked>
      <name>OrgId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Authorization</defaultValue>
      <description></description>
      <id>47fa52c3-c240-44b6-ab42-d14c2817387d</id>
      <masked>false</masked>
      <name>Authorization</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
